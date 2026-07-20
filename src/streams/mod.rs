//! Bidirectional market-data стримы (вариант B: Session + channel).
//!
//! Включается feature `streams`.

mod convert;
mod session;
mod types;

pub(crate) use session::open_session;
pub use session::{
    MarketDataEventReceiver, MarketDataStreamConfig, MarketDataStreamHandle,
    MarketDataStreamSession,
};
pub use types::{MarketDataCommand, MarketDataEvent, MarketDataSubscription, MarketTrade};
