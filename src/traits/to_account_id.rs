use crate::types;

pub trait ToAccountId {
    fn to_account_id(&self) -> types::AccountId;
}

pub trait ToAccountIdRef {
    fn to_account_id_ref(&self) -> &types::AccountId;
}
