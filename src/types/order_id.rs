#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(feature = "lull")]
pub use lull_spec;

#[cfg(feature = "lull")]
impl lull_spec::ReadOrderIdInnerRef<String> for OrderId {
    fn read_order_id_inner_ref(&self) -> &String {
        &self.0
    }
}
