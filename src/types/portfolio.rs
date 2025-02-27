use tinkoff_invest_types as tit;

use crate::types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortfolioPosition {
    /// Идентификатор инструмента
    pub instrument_uid: types::Uid,
    /// Количество инструмента в портфеле в штуках.
    pub quantity_total: Option<types::MoneyValue>,
    pub quantity_blocked: Option<types::MoneyValue>,
}

impl From<&tit::PortfolioPosition> for PortfolioPosition {
    fn from(value: &tit::PortfolioPosition) -> Self {
        Self {
            instrument_uid: value.instrument_uid.as_str().into(),
            quantity_total: value.quantity.map(|x| x.into()),
            quantity_blocked: value.blocked_lots.map(|x| x.into()),
        }
    }
}
