use crate::types;

pub trait ToUid {
    fn to_uid(&self) -> types::Uid;
}
