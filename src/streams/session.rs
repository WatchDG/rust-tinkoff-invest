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
use crate::streams::convert::{command_to_requests, response_to_event};
use crate::streams::types::{MarketDataCommand, MarketDataEvent, MarketDataSubscription};

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
    subscriptions: Vec<MarketDataSubscription>,
}

impl MarketDataStreamSession {
    pub(crate) fn from_parts(
        command_tx: mpsc::Sender<MarketDataCommand>,
        event_rx: mpsc::Receiver<Result<MarketDataEvent, TError>>,
        join: JoinHandle<()>,
    ) -> Self {
        Self {
            command_tx: Some(command_tx),
            event_rx: Some(event_rx),
            join: Some(join),
            subscriptions: Vec::new(),
        }
    }

    /// Подписаться на набор инструментов/каналов.
    pub async fn subscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        self.subscriptions.extend(items.iter().cloned());
        self.send(MarketDataCommand::Subscribe(items)).await
    }

    /// Отписаться.
    pub async fn unsubscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        for item in &items {
            self.subscriptions.retain(|s| s != item);
        }
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

    /// Локально отслеживаемые подписки (оптимистично обновляются при subscribe/unsubscribe).
    pub fn subscriptions(&self) -> &[MarketDataSubscription] {
        &self.subscriptions
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
        let subscriptions = std::mem::take(&mut self.subscriptions);
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
    subscriptions: Vec<MarketDataSubscription>,
}

impl MarketDataStreamHandle {
    pub async fn subscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        self.subscriptions.extend(items.iter().cloned());
        self.send(MarketDataCommand::Subscribe(items)).await
    }

    pub async fn unsubscribe(&mut self, items: Vec<MarketDataSubscription>) -> Result<(), TError> {
        if items.is_empty() {
            return Ok(());
        }
        for item in &items {
            self.subscriptions.retain(|s| s != item);
        }
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

    pub fn subscriptions(&self) -> &[MarketDataSubscription] {
        &self.subscriptions
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

    let forward_pings = config.forward_pings;

    let bridge = tokio::spawn(async move {
        let mut command_rx = command_rx;
        while let Some(command) = command_rx.recv().await {
            for req in command_to_requests(command) {
                if request_tx.send(req).await.is_err() {
                    return;
                }
            }
        }
    });

    let request_stream = ReceiverStream::new(request_rx);
    let request = request_builder(request_stream)?;

    let mut client = client.clone();
    let streaming = client.market_data_stream(request).await?.into_inner();

    let join = tokio::spawn(async move {
        run_reader(streaming, event_tx, forward_pings).await;
        bridge.abort();
    });

    Ok(MarketDataStreamSession::from_parts(
        command_tx, event_rx, join,
    ))
}

async fn run_reader(
    mut streaming: Streaming<MarketDataResponse>,
    event_tx: mpsc::Sender<Result<MarketDataEvent, TError>>,
    forward_pings: bool,
) {
    loop {
        match streaming.message().await {
            Ok(Some(response)) => {
                let event = match response_to_event(response) {
                    Some(MarketDataEvent::Ping) if !forward_pings => continue,
                    Some(event) => Ok(event),
                    None => continue,
                };
                if event_tx.send(event).await.is_err() {
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

fn status_to_error(status: Status) -> TError {
    TError::from(status)
}
