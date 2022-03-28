use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Unspecified,
    Fill,
    Rejected,
    Cancelled,
    New,
    PartiallyFill,
}

impl Into<OrderStatus> for tinkoff_invest_types::OrderExecutionReportStatus {
    fn into(self) -> OrderStatus {
        match self {
            tit::OrderExecutionReportStatus::ExecutionReportStatusUnspecified => {
                OrderStatus::Unspecified
            }
            tit::OrderExecutionReportStatus::ExecutionReportStatusFill => OrderStatus::Fill,
            tit::OrderExecutionReportStatus::ExecutionReportStatusRejected => OrderStatus::Rejected,
            tit::OrderExecutionReportStatus::ExecutionReportStatusCancelled => {
                OrderStatus::Cancelled
            }
            tit::OrderExecutionReportStatus::ExecutionReportStatusNew => OrderStatus::New,
            tit::OrderExecutionReportStatus::ExecutionReportStatusPartiallyfill => {
                OrderStatus::PartiallyFill
            }
        }
    }
}
