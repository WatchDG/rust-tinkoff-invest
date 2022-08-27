#[derive(Debug, PartialEq, Eq)]
pub struct OrderId(String);

impl From<OrderId> for String {
    fn from(value: OrderId) -> Self {
        value.0
    }
}

impl From<String> for OrderId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
