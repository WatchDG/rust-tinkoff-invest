use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum OperationState {
    Unspecified,
    Executed,
    Canceled,
    Progress,
}

impl From<tit::OperationState> for OperationState {
    fn from(value: tit::OperationState) -> Self {
        match value {
            tit::OperationState::Unspecified => OperationState::Unspecified,
            tit::OperationState::Executed => OperationState::Executed,
            tit::OperationState::Canceled => OperationState::Canceled,
            tit::OperationState::Progress => OperationState::Progress,
        }
    }
}

impl From<OperationState> for tit::OperationState {
    fn from(value: OperationState) -> Self {
        match value {
            OperationState::Unspecified => tit::OperationState::Unspecified,
            OperationState::Executed => tit::OperationState::Executed,
            OperationState::Canceled => tit::OperationState::Canceled,
            OperationState::Progress => tit::OperationState::Progress,
        }
    }
}
