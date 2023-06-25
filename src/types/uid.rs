use uuid::Uuid;

use crate::traits;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uid(Uuid);

impl From<&str> for Uid {
    fn from(value: &str) -> Self {
        Uid(Uuid::parse_str(value).unwrap())
    }
}

impl From<Uid> for String {
    fn from(value: Uid) -> Self {
        value.0.to_string()
    }
}

impl traits::ToUid for Uid {
    fn to_uid(&self) -> Uid {
        self.clone()
    }
}

impl traits::ToUidRef for &Uid {
    fn to_uid_ref(&self) -> &Uid {
        self
    }
}
