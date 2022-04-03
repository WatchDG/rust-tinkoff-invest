use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum OperationState {
    Unspecified,
    Executed,
    Canceled,
}

impl From<tit::OperationState> for OperationState {
    fn from(value: tit::OperationState) -> Self {
        match value {
            tinkoff_invest_types::OperationState::Unspecified => OperationState::Unspecified,
            tinkoff_invest_types::OperationState::Executed => OperationState::Executed,
            tinkoff_invest_types::OperationState::Canceled => OperationState::Canceled,
        }
    }
}

impl From<OperationState> for tit::OperationState {
    fn from(value: OperationState) -> Self {
        match value {
            OperationState::Unspecified => tinkoff_invest_types::OperationState::Unspecified,
            OperationState::Executed => tinkoff_invest_types::OperationState::Executed,
            OperationState::Canceled => tinkoff_invest_types::OperationState::Canceled,
        }
    }
}
