use tinkoff_invest_types as tit;

use crate::types;

#[derive(Debug, Clone, PartialEq)]
pub struct PortfolioPosition {
    /// FIGI инструмента.
    pub figi: types::Figi,
    /// Количество инструмента в лотах.
    pub lots: Option<types::MoneyValue>,
}

impl From<tit::PortfolioPosition> for PortfolioPosition {
    fn from(value: tit::PortfolioPosition) -> Self {
        Self {
            figi: value.figi.into(),
            lots: value.quantity_lots.map(|x| x.into()),
        }
    }
}
