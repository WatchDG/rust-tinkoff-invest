use tinkoff_invest_types as tit;

use crate::types;

#[derive(Debug, Clone, PartialEq)]
pub struct Portfolio {
    pub positions: Vec<types::PortfolioPosition>,
}

impl From<tit::PortfolioResponse> for Portfolio {
    fn from(value: tit::PortfolioResponse) -> Self {
        Self {
            positions: value.positions.iter().map(|x| x.clone().into()).collect(),
        }
    }
}
