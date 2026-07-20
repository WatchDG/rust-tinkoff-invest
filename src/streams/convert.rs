use tinkoff_invest_types::{
    CandleInstrument, GetMySubscriptions, InfoInstrument, LastPriceInstrument, MarketDataRequest,
    MarketDataResponse, OrderBookInstrument, PingDelaySettings, PingRequest,
    SubscribeCandlesRequest, SubscribeInfoRequest, SubscribeLastPriceRequest,
    SubscribeOrderBookRequest, SubscribeTradesRequest, SubscriptionAction, TradeInstrument,
    market_data_request, market_data_response,
};

use crate::streams::types::{
    MarketDataCommand, MarketDataEvent, MarketDataSubscription, MarketTrade,
};
use crate::types;

pub(crate) fn command_to_requests(command: MarketDataCommand) -> Vec<MarketDataRequest> {
    match command {
        MarketDataCommand::Subscribe(items) => {
            subscriptions_to_requests(SubscriptionAction::Subscribe, items)
        }
        MarketDataCommand::Unsubscribe(items) => {
            subscriptions_to_requests(SubscriptionAction::Unsubscribe, items)
        }
        MarketDataCommand::GetMySubscriptions => {
            vec![MarketDataRequest {
                payload: Some(market_data_request::Payload::GetMySubscriptions(
                    GetMySubscriptions {},
                )),
            }]
        }
        MarketDataCommand::Ping => vec![MarketDataRequest {
            payload: Some(market_data_request::Payload::Ping(PingRequest::default())),
        }],
        MarketDataCommand::PingSettings { ping_delay_ms } => vec![MarketDataRequest {
            payload: Some(market_data_request::Payload::PingSettings(
                PingDelaySettings {
                    ping_delay_ms: Some(ping_delay_ms),
                },
            )),
        }],
    }
}

fn subscriptions_to_requests(
    action: SubscriptionAction,
    items: Vec<MarketDataSubscription>,
) -> Vec<MarketDataRequest> {
    let mut candles = Vec::new();
    let mut candles_waiting_close = false;
    let mut order_books = Vec::new();
    let mut trades = Vec::new();
    let mut infos = Vec::new();
    let mut last_prices = Vec::new();

    for item in items {
        match item {
            MarketDataSubscription::Candles {
                instrument_uid,
                interval,
                waiting_close,
            } => {
                candles_waiting_close |= waiting_close;
                let mut instrument = CandleInstrument {
                    instrument_id: instrument_uid.into(),
                    ..Default::default()
                };
                instrument.set_interval((&interval).into());
                candles.push(instrument);
            }
            MarketDataSubscription::OrderBook {
                instrument_uid,
                depth,
            } => {
                order_books.push(OrderBookInstrument {
                    instrument_id: instrument_uid.into(),
                    depth,
                    ..Default::default()
                });
            }
            MarketDataSubscription::Trades { instrument_uid } => {
                trades.push(TradeInstrument {
                    instrument_id: instrument_uid.into(),
                    ..Default::default()
                });
            }
            MarketDataSubscription::Info { instrument_uid } => {
                infos.push(InfoInstrument {
                    instrument_id: instrument_uid.into(),
                    ..Default::default()
                });
            }
            MarketDataSubscription::LastPrice { instrument_uid } => {
                last_prices.push(LastPriceInstrument {
                    instrument_id: instrument_uid.into(),
                    ..Default::default()
                });
            }
        }
    }

    let mut requests = Vec::new();

    if !candles.is_empty() {
        let mut req = SubscribeCandlesRequest {
            instruments: candles,
            waiting_close: candles_waiting_close,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(MarketDataRequest {
            payload: Some(market_data_request::Payload::SubscribeCandlesRequest(req)),
        });
    }
    if !order_books.is_empty() {
        let mut req = SubscribeOrderBookRequest {
            instruments: order_books,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(MarketDataRequest {
            payload: Some(market_data_request::Payload::SubscribeOrderBookRequest(req)),
        });
    }
    if !trades.is_empty() {
        let mut req = SubscribeTradesRequest {
            instruments: trades,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(MarketDataRequest {
            payload: Some(market_data_request::Payload::SubscribeTradesRequest(req)),
        });
    }
    if !infos.is_empty() {
        let mut req = SubscribeInfoRequest {
            instruments: infos,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(MarketDataRequest {
            payload: Some(market_data_request::Payload::SubscribeInfoRequest(req)),
        });
    }
    if !last_prices.is_empty() {
        let mut req = SubscribeLastPriceRequest {
            instruments: last_prices,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(MarketDataRequest {
            payload: Some(market_data_request::Payload::SubscribeLastPriceRequest(req)),
        });
    }

    requests
}

pub(crate) fn response_to_event(response: MarketDataResponse) -> Option<MarketDataEvent> {
    let payload = response.payload?;
    Some(match payload {
        market_data_response::Payload::SubscribeCandlesResponse(v) => {
            MarketDataEvent::CandlesSubscriptionAck {
                tracking_id: v.tracking_id,
            }
        }
        market_data_response::Payload::SubscribeOrderBookResponse(v) => {
            MarketDataEvent::OrderBookSubscriptionAck {
                tracking_id: v.tracking_id,
            }
        }
        market_data_response::Payload::SubscribeTradesResponse(v) => {
            MarketDataEvent::TradesSubscriptionAck {
                tracking_id: v.tracking_id,
            }
        }
        market_data_response::Payload::SubscribeInfoResponse(v) => {
            MarketDataEvent::InfoSubscriptionAck {
                tracking_id: v.tracking_id,
            }
        }
        market_data_response::Payload::SubscribeLastPriceResponse(v) => {
            MarketDataEvent::LastPriceSubscriptionAck {
                tracking_id: v.tracking_id,
            }
        }
        market_data_response::Payload::Candle(v) => MarketDataEvent::Candle(v.into()),
        market_data_response::Payload::Orderbook(v) => MarketDataEvent::OrderBook(v.into()),
        market_data_response::Payload::Trade(v) => MarketDataEvent::Trade(MarketTrade {
            instrument_uid: types::Uid::from_api_str(v.instrument_uid.as_str()),
            price: v.price.map(|x| x.into()),
            lots: v.quantity as u64,
            datetime: v.time.map(|x| x.into()),
        }),
        market_data_response::Payload::TradingStatus(v) => MarketDataEvent::TradingStatus {
            instrument_uid: types::Uid::from_api_str(v.instrument_uid.as_str()),
            status: v.trading_status().into(),
        },
        market_data_response::Payload::LastPrice(v) => MarketDataEvent::LastPrice {
            instrument_uid: types::Uid::from_api_str(v.instrument_uid.as_str()),
            price: v.price.map(|x| x.into()),
            datetime: v.time.map(|x| x.into()),
        },
        market_data_response::Payload::Ping(_) => MarketDataEvent::Ping,
        market_data_response::Payload::OpenInterest(_) => return None,
    })
}
