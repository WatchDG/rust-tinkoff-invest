use crate::traits;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderId(String);

impl From<OrderId> for String {
    fn from(value: OrderId) -> Self {
        value.0
    }
}

impl From<&OrderId> for String {
    fn from(value: &OrderId) -> Self {
        value.0.clone()
    }
}

impl From<String> for OrderId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl traits::ToOrderId for OrderId {
    fn to_order_id(&self) -> OrderId {
        self.clone()
    }
}

impl traits::ToOrderIdRef for OrderId {
    fn to_order_id_ref(&self) -> &OrderId {
        self
    }
}
