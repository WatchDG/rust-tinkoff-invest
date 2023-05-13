use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderKind {
    Unspecified,
    Limit,
    Market,
    BestPrice,
}

impl From<tit::OrderType> for OrderKind {
    fn from(value: tit::OrderType) -> Self {
        match value {
            tit::OrderType::Unspecified => OrderKind::Unspecified,
            tit::OrderType::Limit => OrderKind::Limit,
            tit::OrderType::Market => OrderKind::Market,
            tit::OrderType::Bestprice => OrderKind::BestPrice,
        }
    }
}
