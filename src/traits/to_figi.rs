use crate::types;

pub trait ToFigi {
    fn to_figi(&self) -> types::Figi;
}
