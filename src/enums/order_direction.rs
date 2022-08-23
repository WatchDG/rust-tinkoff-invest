use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderDirection {
    Unspecified,
    Buy,
    Sell,
}

impl From<tit::OrderDirection> for OrderDirection {
    fn from(value: tit::OrderDirection) -> Self {
        match value {
            tit::OrderDirection::Unspecified => OrderDirection::Unspecified,
            tit::OrderDirection::Buy => OrderDirection::Buy,
            tit::OrderDirection::Sell => OrderDirection::Sell,
        }
    }
}

impl From<OrderDirection> for tit::OrderDirection {
    fn from(value: OrderDirection) -> Self {
        match value {
            OrderDirection::Unspecified => tit::OrderDirection::Unspecified,
            OrderDirection::Buy => tit::OrderDirection::Buy,
            OrderDirection::Sell => tit::OrderDirection::Sell,
        }
    }
}
