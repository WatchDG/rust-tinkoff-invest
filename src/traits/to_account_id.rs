use crate::types;

pub trait ToAccountId {
    fn to_account_id(&self) -> types::AccountId;
}
