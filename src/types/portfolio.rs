use std::convert::TryFrom;

use tinkoff_invest_types as tit;

use crate::{TError, types};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortfolioPosition {
    /// Идентификатор инструмента
    pub instrument_uid: types::Uid,
    /// Количество инструмента в портфеле в штуках.
    pub quantity_total: Option<types::MoneyValue>,
    pub quantity_blocked: Option<types::MoneyValue>,
}

impl TryFrom<&tit::PortfolioPosition> for PortfolioPosition {
    type Error = TError;

    fn try_from(value: &tit::PortfolioPosition) -> Result<Self, Self::Error> {
        Ok(Self {
            instrument_uid: types::Uid::from_api_str(value.instrument_uid.as_str())?,
            quantity_total: value.quantity.map(|x| x.into()),
            quantity_blocked: value.blocked_lots.map(|x| x.into()),
        })
    }
}
