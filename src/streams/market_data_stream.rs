use std::error::Error;
use tinkoff_invest_types as tit;
use tinkoff_invest_types::market_data_stream_service_client::MarketDataStreamServiceClient;
use tokio::sync::broadcast;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::UnboundedReceiverStream;

use tonic::transport::Endpoint;
use tonic::{service::Interceptor, transport::Channel};

use crate::{enums, traits, TinkoffInvest, TinkoffInvestError};

pub struct MarketDataStreamBuilder<I>
where
    I: Interceptor + Send + 'static,
{
    endpoint: Option<Endpoint>,
    channel: Option<Channel>,
    interceptor: Option<I>,
}

impl<I> MarketDataStreamBuilder<I>
where
    I: Interceptor + Send + 'static,
{
    pub fn new() -> MarketDataStreamBuilder<I> {
        Self {
            endpoint: None,
            channel: None,
            interceptor: None,
        }
    }

    pub async fn build(self) -> Result<MarketDataStream, Box<dyn Error>> {
        let channel = if let Some(channel) = self.channel {
            channel
        } else if let Some(endpoint) = self.endpoint {
            endpoint.connect().await?
        } else {
            return Err(TinkoffInvestError::ChannelNotSet.into());
        };
        let interceptor = self
            .interceptor
            .ok_or(TinkoffInvestError::InterceptorNotSet)?;
        let mut client = MarketDataStreamServiceClient::with_interceptor(channel, interceptor);
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<tit::MarketDataRequest>();
        let receiver_stream = UnboundedReceiverStream::new(receiver);
        let (broadcast_sender, _broadcast_receiver) = broadcast::channel(1);
        let task_broadcast_sender = broadcast_sender.clone();
        let task = tokio::spawn(async move {
            let mut streaming = client
                .market_data_stream(receiver_stream)
                .await
                .unwrap()
                .into_inner();
            while let Some(message) = streaming.message().await.unwrap() {
                if let Some(payload) = message.payload {
                    let data = match payload {
                        tit::market_data_response::Payload::Candle(candlesticks) => Some(
                            enums::MarketDataStreamData::Candlestick(candlesticks.into()),
                        ),
                        tit::market_data_response::Payload::Orderbook(orderbook) => {
                            Some(enums::MarketDataStreamData::Orderbook(orderbook.into()))
                        }
                        _ => None,
                    };
                    if let Some(market_data) = data {
                        task_broadcast_sender.send(market_data).unwrap();
                    }
                }
            }
        });
        let market_data_stream = MarketDataStream {
            sender,
            task,
            broadcast_sender,
        };
        Ok(market_data_stream)
    }
}

impl<I> Default for MarketDataStreamBuilder<I>
where
    I: Interceptor + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<I> From<&TinkoffInvest<I>> for MarketDataStreamBuilder<I>
where
    I: Interceptor + Send + Clone + 'static,
{
    fn from(v: &TinkoffInvest<I>) -> Self {
        Self {
            endpoint: Some(v.endpoint.clone()),
            channel: Some(v.channel.clone()),
            interceptor: Some(v.interceptor.clone()),
        }
    }
}

pub struct MarketDataStream {
    sender: UnboundedSender<tit::MarketDataRequest>,
    broadcast_sender: broadcast::Sender<enums::MarketDataStreamData>,
    pub task: JoinHandle<()>,
}

impl MarketDataStream {
    pub fn subscribe(&self) -> broadcast::Receiver<enums::MarketDataStreamData> {
        self.broadcast_sender.subscribe()
    }

    pub async fn subscribe_candlesticks<T>(
        &mut self,
        instruments: &[T],
        interval: &enums::CandlestickInterval,
    ) -> Result<(), Box<dyn Error>>
    where
        T: traits::ToFigi,
    {
        let mut default_instrument = tit::CandleInstrument::default();
        default_instrument.set_interval(interval.into());

        let instruments = instruments
            .iter()
            .map(|x| {
                let mut instrument = default_instrument.clone();
                instrument.figi = x.to_figi().into();
                instrument
            })
            .collect();

        let mut subscribe_request = tit::SubscribeCandlesRequest::default();
        subscribe_request.set_subscription_action(tit::SubscriptionAction::Subscribe);
        subscribe_request.instruments = instruments;
        subscribe_request.waiting_close = true;

        let payload = tit::market_data_request::Payload::SubscribeCandlesRequest(subscribe_request);

        let request = tit::MarketDataRequest {
            payload: Some(payload),
        };

        self.sender.send(request)?;

        Ok(())
    }

    pub async fn unsubscribe_candlesticks<T>(
        &mut self,
        instruments: &[T],
        interval: &enums::CandlestickInterval,
    ) -> Result<(), Box<dyn Error>>
    where
        T: traits::ToFigi,
    {
        let mut default_instrument = tit::CandleInstrument::default();
        default_instrument.set_interval(interval.into());

        let instruments = instruments
            .iter()
            .map(|x| {
                let mut instrument = default_instrument.clone();
                instrument.figi = x.to_figi().into();
                instrument
            })
            .collect();

        let mut subscribe_request = tit::SubscribeCandlesRequest::default();
        subscribe_request.set_subscription_action(tit::SubscriptionAction::Unsubscribe);
        subscribe_request.instruments = instruments;
        subscribe_request.waiting_close = true;

        let payload = tit::market_data_request::Payload::SubscribeCandlesRequest(subscribe_request);

        let request = tit::MarketDataRequest {
            payload: Some(payload),
        };

        self.sender.send(request)?;

        Ok(())
    }

    pub async fn subscribe_orderbook<T>(
        &mut self,
        instruments: &[T],
        depth: u32,
    ) -> Result<(), Box<dyn Error>>
    where
        T: traits::ToFigi,
    {
        let default_instrument = tit::OrderBookInstrument {
            depth: depth as i32,
            ..Default::default()
        };

        let instruments = instruments
            .iter()
            .map(|x| {
                let mut instrument = default_instrument.clone();
                instrument.figi = x.to_figi().into();
                instrument
            })
            .collect();

        let mut subscribe_request = tit::SubscribeOrderBookRequest::default();
        subscribe_request.set_subscription_action(tit::SubscriptionAction::Subscribe);
        subscribe_request.instruments = instruments;

        let payload =
            tit::market_data_request::Payload::SubscribeOrderBookRequest(subscribe_request);

        let request = tit::MarketDataRequest {
            payload: Some(payload),
        };

        self.sender.send(request)?;

        Ok(())
    }

    pub async fn unsubscribe_orderbook<T>(
        &mut self,
        instruments: &[T],
        depth: u32,
    ) -> Result<(), Box<dyn Error>>
    where
        T: traits::ToFigi,
    {
        let default_instrument = tit::OrderBookInstrument {
            depth: depth as i32,
            ..Default::default()
        };

        let instruments = instruments
            .iter()
            .map(|x| {
                let mut instrument = default_instrument.clone();
                instrument.figi = x.to_figi().into();
                instrument
            })
            .collect();

        let mut subscribe_request = tit::SubscribeOrderBookRequest::default();
        subscribe_request.set_subscription_action(tit::SubscriptionAction::Unsubscribe);
        subscribe_request.instruments = instruments;

        let payload =
            tit::market_data_request::Payload::SubscribeOrderBookRequest(subscribe_request);

        let request = tit::MarketDataRequest {
            payload: Some(payload),
        };

        self.sender.send(request)?;

        Ok(())
    }
}
