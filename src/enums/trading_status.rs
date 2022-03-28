use tinkoff_invest_types;

#[derive(Debug, Clone, PartialEq)]
pub enum TradingStatus {
    Unspecified,
    NotAvailableForTrading,
    OpeningPeriod,
    ClosingPeriod,
    BreakInTrading,
    NormalTrading,
    ClosingAuction,
    DarkPoolAuction,
    DiscreteAuction,
    OpeningAuctionPeriod,
    TradingAtClosingAuctionPrice,
    SessionAssigned,
    SessionClose,
    SessionOpen,
    DealerNormalTrading,
    DealerBreakInTrading,
    DealerNotAvailableForTrading,
}

impl From<tinkoff_invest_types::SecurityTradingStatus> for TradingStatus {
    fn from(trading_status: tinkoff_invest_types::SecurityTradingStatus) -> Self {
        match trading_status {
            tinkoff_invest_types::SecurityTradingStatus::Unspecified => TradingStatus::Unspecified,
            tinkoff_invest_types::SecurityTradingStatus::NotAvailableForTrading => {
                TradingStatus::NotAvailableForTrading
            }
            tinkoff_invest_types::SecurityTradingStatus::OpeningPeriod => {
                TradingStatus::OpeningPeriod
            }
            tinkoff_invest_types::SecurityTradingStatus::ClosingPeriod => {
                TradingStatus::ClosingPeriod
            }
            tinkoff_invest_types::SecurityTradingStatus::BreakInTrading => {
                TradingStatus::BreakInTrading
            }
            tinkoff_invest_types::SecurityTradingStatus::NormalTrading => {
                TradingStatus::NormalTrading
            }
            tinkoff_invest_types::SecurityTradingStatus::ClosingAuction => {
                TradingStatus::ClosingAuction
            }
            tinkoff_invest_types::SecurityTradingStatus::DarkPoolAuction => {
                TradingStatus::DarkPoolAuction
            }
            tinkoff_invest_types::SecurityTradingStatus::DiscreteAuction => {
                TradingStatus::DiscreteAuction
            }
            tinkoff_invest_types::SecurityTradingStatus::OpeningAuctionPeriod => {
                TradingStatus::OpeningAuctionPeriod
            }
            tinkoff_invest_types::SecurityTradingStatus::TradingAtClosingAuctionPrice => {
                TradingStatus::TradingAtClosingAuctionPrice
            }
            tinkoff_invest_types::SecurityTradingStatus::SessionAssigned => {
                TradingStatus::SessionAssigned
            }
            tinkoff_invest_types::SecurityTradingStatus::SessionClose => {
                TradingStatus::SessionClose
            }
            tinkoff_invest_types::SecurityTradingStatus::SessionOpen => TradingStatus::SessionOpen,
            tinkoff_invest_types::SecurityTradingStatus::DealerNormalTrading => {
                TradingStatus::DealerNormalTrading
            }
            tinkoff_invest_types::SecurityTradingStatus::DealerBreakInTrading => {
                TradingStatus::DealerBreakInTrading
            }
            tinkoff_invest_types::SecurityTradingStatus::DealerNotAvailableForTrading => {
                TradingStatus::DealerNotAvailableForTrading
            }
        }
    }
}
