use crate::types;

pub trait ToUid {
    fn to_uid(&self) -> types::Uid;
}

pub trait ToUidRef {
    fn to_uid_ref(&self) -> &types::Uid;
}
