use crate::enums;

pub trait ToInstrumentType {
    fn to_instrument_type(&self) -> enums::InstrumentType;
}
