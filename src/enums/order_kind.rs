use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderKind {
    Unspecified,
    Limit,
    Market,
}

impl Into<OrderKind> for tit::OrderType {
    fn into(self) -> OrderKind {
        match self {
            tit::OrderType::Unspecified => OrderKind::Unspecified,
            tit::OrderType::Limit => OrderKind::Limit,
            tit::OrderType::Market => OrderKind::Market,
        }
    }
}
