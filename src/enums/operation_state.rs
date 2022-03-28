use tinkoff_invest_types;

#[derive(Debug, Clone, PartialEq)]
pub enum OperationState {
    Unspecified,
    Executed,
    Canceled,
}

impl Into<tinkoff_invest_types::OperationState> for OperationState {
    fn into(self) -> tinkoff_invest_types::OperationState {
        match self {
            OperationState::Unspecified => tinkoff_invest_types::OperationState::Unspecified,
            OperationState::Executed => tinkoff_invest_types::OperationState::Executed,
            OperationState::Canceled => tinkoff_invest_types::OperationState::Canceled,
        }
    }
}

impl Into<OperationState> for tinkoff_invest_types::OperationState {
    fn into(self) -> OperationState {
        match self {
            tinkoff_invest_types::OperationState::Unspecified => OperationState::Unspecified,
            tinkoff_invest_types::OperationState::Executed => OperationState::Executed,
            tinkoff_invest_types::OperationState::Canceled => OperationState::Canceled,
        }
    }
}
