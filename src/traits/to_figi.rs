use crate::types;

pub trait ToFigi {
    fn to_figi(&self) -> types::Figi;
}

pub trait ToFigiRef {
    fn to_figi_ref(&self) -> &types::Figi;
}
