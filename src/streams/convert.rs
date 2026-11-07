use std::convert::TryFrom;

use tinkoff_invest_types::{
    CandleInstrument, CandleSubscription, GetMySubscriptions, InfoInstrument, InfoSubscription,
    LastPriceInstrument, LastPriceSubscription, MarketDataRequest, MarketDataResponse,
    OrderBookInstrument, OrderBookSubscription, PingDelaySettings, PingRequest,
    SubscribeCandlesRequest, SubscribeInfoRequest, SubscribeLastPriceRequest,
    SubscribeOrderBookRequest, SubscribeTradesRequest, SubscriptionAction, SubscriptionStatus,
    TradeInstrument, TradeSubscription, market_data_request, market_data_response,
};

use crate::streams::types::{
    MarketDataCommand, MarketDataEvent, MarketDataSubscription, MarketTrade,
    SubscriptionInstrumentStatus,
};
use crate::{TError, types};

pub(crate) enum SubIntent {
    Subscribe(Vec<MarketDataSubscription>),
    Unsubscribe(Vec<MarketDataSubscription>),
}

pub(crate) struct Outgoing {
    pub request: MarketDataRequest,
    pub intent: Option<SubIntent>,
}

pub(crate) fn command_to_requests(command: MarketDataCommand) -> Result<Vec<Outgoing>, TError> {
    match command {
        MarketDataCommand::Subscribe(items) => {
            subscriptions_to_requests(SubscriptionAction::Subscribe, true, items)
        }
        MarketDataCommand::Unsubscribe(items) => {
            subscriptions_to_requests(SubscriptionAction::Unsubscribe, false, items)
        }
        MarketDataCommand::GetMySubscriptions => Ok(vec![Outgoing {
            request: MarketDataRequest {
                payload: Some(market_data_request::Payload::GetMySubscriptions(
                    GetMySubscriptions {},
                )),
            },
            intent: None,
        }]),
        MarketDataCommand::Ping => Ok(vec![Outgoing {
            request: MarketDataRequest {
                payload: Some(market_data_request::Payload::Ping(PingRequest::default())),
            },
            intent: None,
        }]),
        MarketDataCommand::PingSettings { ping_delay_ms } => Ok(vec![Outgoing {
            request: MarketDataRequest {
                payload: Some(market_data_request::Payload::PingSettings(
                    PingDelaySettings {
                        ping_delay_ms: Some(ping_delay_ms),
                    },
                )),
            },
            intent: None,
        }]),
    }
}

fn subscriptions_to_requests(
    action: SubscriptionAction,
    subscribe: bool,
    items: Vec<MarketDataSubscription>,
) -> Result<Vec<Outgoing>, TError> {
    let mut candles_close = Vec::new();
    let mut candles_open = Vec::new();
    let mut order_books = Vec::new();
    let mut trades = Vec::new();
    let mut infos = Vec::new();
    let mut last_prices = Vec::new();

    for item in items {
        match &item {
            MarketDataSubscription::Candles {
                instrument_uid,
                interval,
                waiting_close,
            } => {
                let mut instrument = CandleInstrument {
                    instrument_id: instrument_uid.clone().into(),
                    ..Default::default()
                };
                instrument.set_interval(interval.try_into()?);
                if *waiting_close {
                    candles_close.push((item.clone(), instrument));
                } else {
                    candles_open.push((item.clone(), instrument));
                }
            }
            MarketDataSubscription::OrderBook {
                instrument_uid,
                depth,
            } => {
                order_books.push((
                    item.clone(),
                    OrderBookInstrument {
                        instrument_id: instrument_uid.clone().into(),
                        depth: *depth,
                        ..Default::default()
                    },
                ));
            }
            MarketDataSubscription::Trades { instrument_uid } => {
                trades.push((
                    item.clone(),
                    TradeInstrument {
                        instrument_id: instrument_uid.clone().into(),
                        ..Default::default()
                    },
                ));
            }
            MarketDataSubscription::Info { instrument_uid } => {
                infos.push((
                    item.clone(),
                    InfoInstrument {
                        instrument_id: instrument_uid.clone().into(),
                        ..Default::default()
                    },
                ));
            }
            MarketDataSubscription::LastPrice { instrument_uid } => {
                last_prices.push((
                    item.clone(),
                    LastPriceInstrument {
                        instrument_id: instrument_uid.clone().into(),
                        ..Default::default()
                    },
                ));
            }
        }
    }

    let mut requests = Vec::new();
    push_candles(&mut requests, action, subscribe, candles_close, true);
    push_candles(&mut requests, action, subscribe, candles_open, false);
    if !order_books.is_empty() {
        let (subs, instruments) = unzip(order_books);
        let mut req = SubscribeOrderBookRequest {
            instruments,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(outgoing(
            subscribe,
            subs,
            market_data_request::Payload::SubscribeOrderBookRequest(req),
        ));
    }
    if !trades.is_empty() {
        let (subs, instruments) = unzip(trades);
        let mut req = SubscribeTradesRequest {
            instruments,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(outgoing(
            subscribe,
            subs,
            market_data_request::Payload::SubscribeTradesRequest(req),
        ));
    }
    if !infos.is_empty() {
        let (subs, instruments) = unzip(infos);
        let mut req = SubscribeInfoRequest {
            instruments,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(outgoing(
            subscribe,
            subs,
            market_data_request::Payload::SubscribeInfoRequest(req),
        ));
    }
    if !last_prices.is_empty() {
        let (subs, instruments) = unzip(last_prices);
        let mut req = SubscribeLastPriceRequest {
            instruments,
            ..Default::default()
        };
        req.set_subscription_action(action);
        requests.push(outgoing(
            subscribe,
            subs,
            market_data_request::Payload::SubscribeLastPriceRequest(req),
        ));
    }

    Ok(requests)
}

fn push_candles(
    requests: &mut Vec<Outgoing>,
    action: SubscriptionAction,
    subscribe: bool,
    group: Vec<(MarketDataSubscription, CandleInstrument)>,
    waiting_close: bool,
) {
    if group.is_empty() {
        return;
    }
    let (subs, instruments) = unzip(group);
    let mut req = SubscribeCandlesRequest {
        instruments,
        waiting_close,
        ..Default::default()
    };
    req.set_subscription_action(action);
    requests.push(outgoing(
        subscribe,
        subs,
        market_data_request::Payload::SubscribeCandlesRequest(req),
    ));
}

fn outgoing(
    subscribe: bool,
    items: Vec<MarketDataSubscription>,
    payload: market_data_request::Payload,
) -> Outgoing {
    let intent = if subscribe {
        SubIntent::Subscribe(items)
    } else {
        SubIntent::Unsubscribe(items)
    };
    Outgoing {
        request: MarketDataRequest {
            payload: Some(payload),
        },
        intent: Some(intent),
    }
}

fn unzip<T>(pairs: Vec<(MarketDataSubscription, T)>) -> (Vec<MarketDataSubscription>, Vec<T>) {
    let mut subs = Vec::with_capacity(pairs.len());
    let mut values = Vec::with_capacity(pairs.len());
    for (sub, value) in pairs {
        subs.push(sub);
        values.push(value);
    }
    (subs, values)
}

pub(crate) fn response_to_event(
    response: MarketDataResponse,
) -> Result<Option<MarketDataEvent>, TError> {
    let Some(payload) = response.payload else {
        return Ok(None);
    };
    Ok(Some(match payload {
        market_data_response::Payload::SubscribeCandlesResponse(v) => {
            MarketDataEvent::CandlesSubscriptionAck {
                tracking_id: v.tracking_id,
                instruments: candle_statuses(v.candles_subscriptions)?,
            }
        }
        market_data_response::Payload::SubscribeOrderBookResponse(v) => {
            MarketDataEvent::OrderBookSubscriptionAck {
                tracking_id: v.tracking_id,
                instruments: order_book_statuses(v.order_book_subscriptions)?,
            }
        }
        market_data_response::Payload::SubscribeTradesResponse(v) => {
            MarketDataEvent::TradesSubscriptionAck {
                tracking_id: v.tracking_id,
                instruments: trade_statuses(v.trade_subscriptions)?,
            }
        }
        market_data_response::Payload::SubscribeInfoResponse(v) => {
            MarketDataEvent::InfoSubscriptionAck {
                tracking_id: v.tracking_id,
                instruments: info_statuses(v.info_subscriptions)?,
            }
        }
        market_data_response::Payload::SubscribeLastPriceResponse(v) => {
            MarketDataEvent::LastPriceSubscriptionAck {
                tracking_id: v.tracking_id,
                instruments: last_price_statuses(v.last_price_subscriptions)?,
            }
        }
        market_data_response::Payload::Candle(v) => MarketDataEvent::Candle(v.try_into()?),
        market_data_response::Payload::Orderbook(v) => MarketDataEvent::OrderBook(v.try_into()?),
        market_data_response::Payload::Trade(v) => MarketDataEvent::Trade(MarketTrade {
            instrument_uid: types::Uid::from_api_str(v.instrument_uid.as_str())?,
            price: v.price.map(|x| x.into()),
            lots: crate::num::to_u64(v.quantity)?,
            datetime: v.time.map(types::DateTime::try_from).transpose()?,
        }),
        market_data_response::Payload::TradingStatus(v) => MarketDataEvent::TradingStatus {
            instrument_uid: types::Uid::from_api_str(v.instrument_uid.as_str())?,
            status: v.trading_status().into(),
        },
        market_data_response::Payload::LastPrice(v) => MarketDataEvent::LastPrice {
            instrument_uid: types::Uid::from_api_str(v.instrument_uid.as_str())?,
            price: v.price.map(|x| x.into()),
            datetime: v.time.map(types::DateTime::try_from).transpose()?,
        },
        market_data_response::Payload::Ping(_) => MarketDataEvent::Ping,
        market_data_response::Payload::OpenInterest(_) => return Ok(None),
    }))
}

fn status_row(
    instrument_uid: &str,
    status: SubscriptionStatus,
    interval: Option<crate::enums::CandlestickInterval>,
    depth: Option<i32>,
) -> Result<SubscriptionInstrumentStatus, TError> {
    Ok(SubscriptionInstrumentStatus {
        instrument_uid: types::Uid::from_api_str(instrument_uid)?,
        success: status == SubscriptionStatus::Success,
        status: format!("{status:?}"),
        interval,
        depth,
    })
}

fn candle_statuses(
    subs: Vec<CandleSubscription>,
) -> Result<Vec<SubscriptionInstrumentStatus>, TError> {
    let mut out = Vec::with_capacity(subs.len());
    for sub in subs {
        out.push(status_row(
            sub.instrument_uid.as_str(),
            sub.subscription_status(),
            Some(sub.interval().into()),
            None,
        )?);
    }
    Ok(out)
}

fn order_book_statuses(
    subs: Vec<OrderBookSubscription>,
) -> Result<Vec<SubscriptionInstrumentStatus>, TError> {
    let mut out = Vec::with_capacity(subs.len());
    for sub in subs {
        out.push(status_row(
            sub.instrument_uid.as_str(),
            sub.subscription_status(),
            None,
            Some(sub.depth),
        )?);
    }
    Ok(out)
}

fn trade_statuses(
    subs: Vec<TradeSubscription>,
) -> Result<Vec<SubscriptionInstrumentStatus>, TError> {
    let mut out = Vec::with_capacity(subs.len());
    for sub in subs {
        let status = sub.subscription_status();
        out.push(status_row(sub.instrument_uid.as_str(), status, None, None)?);
    }
    Ok(out)
}

fn info_statuses(subs: Vec<InfoSubscription>) -> Result<Vec<SubscriptionInstrumentStatus>, TError> {
    let mut out = Vec::with_capacity(subs.len());
    for sub in subs {
        let status = sub.subscription_status();
        out.push(status_row(sub.instrument_uid.as_str(), status, None, None)?);
    }
    Ok(out)
}

fn last_price_statuses(
    subs: Vec<LastPriceSubscription>,
) -> Result<Vec<SubscriptionInstrumentStatus>, TError> {
    let mut out = Vec::with_capacity(subs.len());
    for sub in subs {
        let status = sub.subscription_status();
        out.push(status_row(sub.instrument_uid.as_str(), status, None, None)?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::CandlestickInterval;
    use crate::types::Uid;

    fn uid() -> Uid {
        Uid::try_from("550e8400-e29b-41d4-a716-446655440000").unwrap()
    }

    #[test]
    fn waiting_close_splits_requests() {
        let outgoing = command_to_requests(MarketDataCommand::Subscribe(vec![
            MarketDataSubscription::Candles {
                instrument_uid: uid(),
                interval: CandlestickInterval::Min,
                waiting_close: true,
            },
            MarketDataSubscription::Candles {
                instrument_uid: uid(),
                interval: CandlestickInterval::Min5,
                waiting_close: false,
            },
        ]))
        .unwrap();
        assert_eq!(outgoing.len(), 2);
        let flags: Vec<bool> = outgoing
            .iter()
            .map(|item| match &item.request.payload {
                Some(market_data_request::Payload::SubscribeCandlesRequest(req)) => {
                    req.waiting_close
                }
                other => panic!("unexpected payload {other:?}"),
            })
            .collect();
        assert!(flags.contains(&true));
        assert!(flags.contains(&false));
    }
}
