use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::{Status, Streaming};

use tinkoff_invest_types::market_data_stream_service_client::MarketDataStreamServiceClient;
use tinkoff_invest_types::{MarketDataRequest, MarketDataResponse};

use crate::TError;
use crate::streams::convert::{SubIntent, command_to_requests, response_to_event};
use crate::streams::types::{
    MarketDataCommand, MarketDataEvent, MarketDataSubscription, SubscriptionInstrumentStatus,
};

/// Конфиг bidirectional market-data сессии.
#[derive(Debug, Clone)]
pub struct MarketDataStreamConfig {
    /// Ёмкость канала событий.
    pub event_buffer: usize,
    /// Ёмкость очереди исходящих команд.
    pub command_buffer: usize,
    /// Пробрасывать ли [`MarketDataEvent::Ping`] наружу.
    pub forward_pings: bool,
}

impl Default for MarketDataStreamConfig {
    fn default() -> Self {
        Self {
            event_buffer: 256,
            command_buffer: 32,
            forward_pings: false,
        }
    }
}

/// Управляемая bidirectional-сессия `MarketDataStream`.
pub struct MarketDataStreamSession {
    command_tx: Option<mpsc::Sender<MarketDataCommand>>,
    event_rx: Option<mpsc::Receiver<Result<MarketDataEvent, TError>>>,
    join: Option<JoinHandle<()>>,
    subscriptions: Arc<Mutex<Vec<MarketDataSubscription>>>,
}

impl MarketDataStreamSession {
    pub(crate) fn from_parts(
        command_tx: mpsc::Sender<MarketDataCommand>,
        event_rx: mpsc::Receiver<Result<MarketDataEvent, TError>>,
        join: JoinHandle<()>,
        subscriptions: Arc<Mutex<Vec<MarketDataSubscription>>>,
    ) -> Self {
        Self {
            command_tx: Some(command_tx),
            event_rx: Some(event_rx),
            join: Some(join),
            subscriptions,
        }
    }

    /// Подписаться на набор инструментов/каналов.
    pub async fn subscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        validate_subscriptions(&items)?;
        self.send(MarketDataCommand::Subscribe(items)).await
    }

    /// Отписаться.
    pub async fn unsubscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        validate_subscriptions(&items)?;
        self.send(MarketDataCommand::Unsubscribe(items)).await
    }

    /// Запросить текущие подписки на стороне сервера.
    pub async fn get_my_subscriptions(&self) -> Result<(), TError> {
        self.send(MarketDataCommand::GetMySubscriptions).await
    }

    /// Отправить ping.
    pub async fn ping(&self) -> Result<(), TError> {
        self.send(MarketDataCommand::Ping).await
    }

    /// Настроить интервал ping (мс).
    pub async fn set_ping_delay_ms(&self, ping_delay_ms: i32) -> Result<(), TError> {
        self.send(MarketDataCommand::PingSettings { ping_delay_ms })
            .await
    }

    /// Подписки, которые сервер подтвердил ack-ом.
    pub fn subscriptions(&self) -> Vec<MarketDataSubscription> {
        self.subscriptions.lock().expect("subscriptions").clone()
    }

    /// Следующее событие. `None` — стрим завершён.
    pub async fn recv(&mut self) -> Option<Result<MarketDataEvent, TError>> {
        self.event_rx.as_mut()?.recv().await
    }

    /// Разделить управление и приём событий.
    pub fn split(mut self) -> (MarketDataStreamHandle, MarketDataEventReceiver) {
        let command_tx = self.command_tx.take().expect("session command_tx");
        let event_rx = self.event_rx.take().expect("session event_rx");
        let join = self.join.take().expect("session join");
        let subscriptions = Arc::clone(&self.subscriptions);
        (
            MarketDataStreamHandle {
                command_tx,
                subscriptions,
            },
            MarketDataEventReceiver { event_rx, join },
        )
    }

    async fn send(&self, command: MarketDataCommand) -> Result<(), TError> {
        self.command_tx
            .as_ref()
            .ok_or(TError::StreamClosed)?
            .send(command)
            .await
            .map_err(|_| TError::StreamClosed)
    }
}

impl Drop for MarketDataStreamSession {
    fn drop(&mut self) {
        if let Some(join) = self.join.take() {
            join.abort();
        }
    }
}

/// Handle для отправки команд из другой задачи.
pub struct MarketDataStreamHandle {
    command_tx: mpsc::Sender<MarketDataCommand>,
    subscriptions: Arc<Mutex<Vec<MarketDataSubscription>>>,
}

impl MarketDataStreamHandle {
    pub async fn subscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        validate_subscriptions(&items)?;
        self.send(MarketDataCommand::Subscribe(items)).await
    }

    pub async fn unsubscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        validate_subscriptions(&items)?;
        self.send(MarketDataCommand::Unsubscribe(items)).await
    }

    pub async fn get_my_subscriptions(&self) -> Result<(), TError> {
        self.send(MarketDataCommand::GetMySubscriptions).await
    }

    pub async fn ping(&self) -> Result<(), TError> {
        self.send(MarketDataCommand::Ping).await
    }

    pub async fn set_ping_delay_ms(&self, ping_delay_ms: i32) -> Result<(), TError> {
        self.send(MarketDataCommand::PingSettings { ping_delay_ms })
            .await
    }

    pub fn subscriptions(&self) -> Vec<MarketDataSubscription> {
        self.subscriptions.lock().expect("subscriptions").clone()
    }

    async fn send(&self, command: MarketDataCommand) -> Result<(), TError> {
        self.command_tx
            .send(command)
            .await
            .map_err(|_| TError::StreamClosed)
    }
}

/// Приёмник событий после [`MarketDataStreamSession::split`].
pub struct MarketDataEventReceiver {
    event_rx: mpsc::Receiver<Result<MarketDataEvent, TError>>,
    join: JoinHandle<()>,
}

impl MarketDataEventReceiver {
    pub async fn recv(&mut self) -> Option<Result<MarketDataEvent, TError>> {
        self.event_rx.recv().await
    }
}

impl Drop for MarketDataEventReceiver {
    fn drop(&mut self) {
        self.join.abort();
    }
}

pub(crate) async fn open_session<I>(
    client: &MarketDataStreamServiceClient<InterceptedService<Channel, I>>,
    request_builder: impl FnOnce(
        ReceiverStream<MarketDataRequest>,
    )
        -> Result<tonic::Request<ReceiverStream<MarketDataRequest>>, TError>,
    config: MarketDataStreamConfig,
) -> Result<MarketDataStreamSession, TError>
where
    I: Interceptor + Clone + Send + 'static,
{
    let (command_tx, command_rx) = mpsc::channel::<MarketDataCommand>(config.command_buffer.max(1));
    let (request_tx, request_rx) = mpsc::channel::<MarketDataRequest>(config.command_buffer.max(1));
    let (event_tx, event_rx) =
        mpsc::channel::<Result<MarketDataEvent, TError>>(config.event_buffer.max(1));
    let (intent_tx, intent_rx) = mpsc::unbounded_channel::<SubIntent>();
    let subscriptions = Arc::new(Mutex::new(Vec::new()));

    let forward_pings = config.forward_pings;
    let bridge_events = event_tx.clone();

    let bridge = tokio::spawn(async move {
        let mut command_rx = command_rx;
        while let Some(command) = command_rx.recv().await {
            let outgoing = match command_to_requests(command) {
                Ok(outgoing) => outgoing,
                Err(err) => {
                    if bridge_events.send(Err(err)).await.is_err() {
                        return;
                    }
                    continue;
                }
            };
            for item in outgoing {
                if request_tx.send(item.request).await.is_err() {
                    return;
                }
                if let Some(intent) = item.intent {
                    if intent_tx.send(intent).is_err() {
                        return;
                    }
                }
            }
        }
    });

    let request_stream = ReceiverStream::new(request_rx);
    let request = request_builder(request_stream)?;

    let mut client = client.clone();
    let streaming = client.market_data_stream(request).await?.into_inner();
    let book = Arc::clone(&subscriptions);

    let join = tokio::spawn(async move {
        run_reader(streaming, event_tx, intent_rx, book, forward_pings).await;
        bridge.abort();
    });

    Ok(MarketDataStreamSession::from_parts(
        command_tx,
        event_rx,
        join,
        subscriptions,
    ))
}

async fn run_reader(
    mut streaming: Streaming<MarketDataResponse>,
    event_tx: mpsc::Sender<Result<MarketDataEvent, TError>>,
    mut intent_rx: mpsc::UnboundedReceiver<SubIntent>,
    book: Arc<Mutex<Vec<MarketDataSubscription>>>,
    forward_pings: bool,
) {
    loop {
        match streaming.message().await {
            Ok(Some(response)) => {
                let mut event = match response_to_event(response) {
                    Ok(Some(MarketDataEvent::Ping)) if !forward_pings => continue,
                    Ok(Some(event)) => event,
                    Ok(None) => continue,
                    Err(err) => {
                        if event_tx.send(Err(err)).await.is_err() {
                            break;
                        }
                        continue;
                    }
                };
                if is_subscription_ack(&event) {
                    if let Some(intent) = intent_rx.recv().await {
                        apply_intent(&book, intent, &event);
                    }
                }
                if let MarketDataEvent::Candle(candle) = &mut event {
                    mark_waiting_close(candle, &book);
                }
                if event_tx.send(Ok(event)).await.is_err() {
                    break;
                }
            }
            Ok(None) => break,
            Err(status) => {
                let _ = event_tx.send(Err(status_to_error(status))).await;
                break;
            }
        }
    }
}

fn is_subscription_ack(event: &MarketDataEvent) -> bool {
    matches!(
        event,
        MarketDataEvent::CandlesSubscriptionAck { .. }
            | MarketDataEvent::OrderBookSubscriptionAck { .. }
            | MarketDataEvent::TradesSubscriptionAck { .. }
            | MarketDataEvent::InfoSubscriptionAck { .. }
            | MarketDataEvent::LastPriceSubscriptionAck { .. }
    )
}

fn apply_intent(
    book: &Mutex<Vec<MarketDataSubscription>>,
    intent: SubIntent,
    event: &MarketDataEvent,
) {
    let Some(statuses) = ack_statuses(event) else {
        return;
    };
    let mut book = book.lock().expect("subscriptions");
    match intent {
        SubIntent::Subscribe(items) => {
            for item in items {
                if item_accepted(&item, statuses) {
                    if !book.contains(&item) {
                        book.push(item);
                    }
                } else {
                    book.retain(|stored| stored != &item);
                }
            }
        }
        SubIntent::Unsubscribe(items) => {
            for item in items {
                if item_accepted(&item, statuses) {
                    book.retain(|stored| stored != &item);
                } else if !book.contains(&item) {
                    book.push(item);
                }
            }
        }
    }
}

fn ack_statuses(event: &MarketDataEvent) -> Option<&[SubscriptionInstrumentStatus]> {
    match event {
        MarketDataEvent::CandlesSubscriptionAck { instruments, .. }
        | MarketDataEvent::OrderBookSubscriptionAck { instruments, .. }
        | MarketDataEvent::TradesSubscriptionAck { instruments, .. }
        | MarketDataEvent::InfoSubscriptionAck { instruments, .. }
        | MarketDataEvent::LastPriceSubscriptionAck { instruments, .. } => Some(instruments),
        _ => None,
    }
}

fn item_accepted(item: &MarketDataSubscription, statuses: &[SubscriptionInstrumentStatus]) -> bool {
    statuses
        .iter()
        .any(|status| status.success && status_matches(item, status))
}

fn status_matches(item: &MarketDataSubscription, status: &SubscriptionInstrumentStatus) -> bool {
    match item {
        MarketDataSubscription::Candles {
            instrument_uid,
            interval,
            ..
        } => {
            &status.instrument_uid == instrument_uid
                && status
                    .interval
                    .as_ref()
                    .is_none_or(|value| value == interval)
        }
        MarketDataSubscription::OrderBook {
            instrument_uid,
            depth,
        } => {
            &status.instrument_uid == instrument_uid
                && status.depth.is_none_or(|value| value == *depth)
        }
        MarketDataSubscription::Trades { instrument_uid }
        | MarketDataSubscription::Info { instrument_uid }
        | MarketDataSubscription::LastPrice { instrument_uid } => {
            &status.instrument_uid == instrument_uid
        }
    }
}

fn mark_waiting_close(
    candle: &mut crate::types::Candlestick,
    book: &Mutex<Vec<MarketDataSubscription>>,
) {
    let book = book.lock().expect("subscriptions");
    let complete = book.iter().any(|sub| match sub {
        MarketDataSubscription::Candles {
            instrument_uid,
            interval,
            waiting_close: true,
        } => {
            instrument_uid == candle.instrument_uid.as_ref() && interval == candle.interval.as_ref()
        }
        _ => false,
    });
    if complete {
        candle.is_complete = true;
    }
}

fn validate_subscriptions(items: &[MarketDataSubscription]) -> Result<(), TError> {
    for item in items {
        if let MarketDataSubscription::Candles { interval, .. } = item {
            let _: tinkoff_invest_types::SubscriptionInterval = interval.try_into()?;
        }
    }
    Ok(())
}

fn status_to_error(status: Status) -> TError {
    TError::from(status)
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::{MarketDataSubscription, mark_waiting_close};
    use crate::enums::CandlestickInterval;
    use crate::types::{Candlestick, DateTime, Uid};

    #[test]
    fn waiting_close_marks_stream_candle_complete() {
        let uid = Uid::try_from("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let mut candle = Candlestick {
            instrument_uid: Arc::new(uid.clone()),
            interval: Arc::new(CandlestickInterval::Min),
            datetime: DateTime {
                seconds: 0,
                nanoseconds: 0,
            },
            open: None,
            high: None,
            low: None,
            close: None,
            volume: 1,
            is_complete: false,
        };
        let book = Mutex::new(vec![MarketDataSubscription::Candles {
            instrument_uid: uid,
            interval: CandlestickInterval::Min,
            waiting_close: true,
        }]);
        mark_waiting_close(&mut candle, &book);
        assert!(candle.is_complete);
    }
}
