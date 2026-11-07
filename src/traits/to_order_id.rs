use crate::types;

pub trait ToOrderId {
    fn to_order_id(&self) -> types::OrderId;
}

pub trait ToOrderIdRef {
    fn to_order_id_ref(&self) -> &types::OrderId;
}
