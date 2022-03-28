use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderDirection {
    Unspecified,
    Buy,
    Sell,
}

impl Into<OrderDirection> for tinkoff_invest_types::OrderDirection {
    fn into(self) -> OrderDirection {
        match self {
            tit::OrderDirection::Unspecified => OrderDirection::Unspecified,
            tit::OrderDirection::Buy => OrderDirection::Buy,
            tit::OrderDirection::Sell => OrderDirection::Sell,
        }
    }
}

impl Into<tinkoff_invest_types::OrderDirection> for OrderDirection {
    fn into(self) -> tinkoff_invest_types::OrderDirection {
        match self {
            OrderDirection::Unspecified => tit::OrderDirection::Unspecified,
            OrderDirection::Buy => tit::OrderDirection::Buy,
            OrderDirection::Sell => tit::OrderDirection::Sell,
        }
    }
}
