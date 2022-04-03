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

impl From<tit::OrderExecutionReportStatus> for OrderStatus {
    fn from(value: tit::OrderExecutionReportStatus) -> Self {
        match value {
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
