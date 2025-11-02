use uuid::Uuid;

/// Контекст вызова, содержащий информацию для идентификации запроса
#[derive(Debug, Clone)]
pub struct TinkoffInvestCallContext {
    /// Идентификатор запроса (x-tracking-id)
    pub request_id: String,
}

impl TinkoffInvestCallContext {
    /// Создает новый контекст с заданным request_id или автоматически генерирует его
    pub fn new(request_id: Option<String>) -> Self {
        Self {
            request_id: request_id.unwrap_or_else(|| Uuid::now_v7().to_string()),
        }
    }
}
