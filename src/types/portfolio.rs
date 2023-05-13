use tinkoff_invest_types as tit;

use crate::types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortfolioPosition {
    /// FIGI инструмента.
    pub figi: types::Figi,
    /// Количество инструмента в лотах.
    // Количество инструмента в портфеле в штуках.
    pub quantity: Option<types::MoneyValue>,
}

impl From<tit::PortfolioPosition> for PortfolioPosition {
    fn from(value: tit::PortfolioPosition) -> Self {
        Self {
            figi: value.figi.into(),
            quantity: value.quantity.map(|x| x.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Portfolio {
    pub positions: Vec<PortfolioPosition>,
}

impl From<tit::PortfolioResponse> for Portfolio {
    fn from(value: tit::PortfolioResponse) -> Self {
        Self {
            positions: value.positions.iter().map(|x| x.clone().into()).collect(),
        }
    }
}
