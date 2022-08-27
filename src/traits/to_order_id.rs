use crate::types;

pub trait ToOrderId {
    fn to_order_id(&self) -> types::OrderId;
}
