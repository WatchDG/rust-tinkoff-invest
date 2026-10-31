use std::fmt::Display;

use crate::TError;

pub(crate) fn to_u64<T>(value: T) -> Result<u64, TError>
where
    T: TryInto<u64> + Copy + Display,
{
    value
        .try_into()
        .map_err(|_| TError::InvalidQuantity(value.to_string()))
}

pub(crate) fn to_u32<T>(value: T) -> Result<u32, TError>
where
    T: TryInto<u32> + Copy + Display,
{
    value
        .try_into()
        .map_err(|_| TError::InvalidQuantity(value.to_string()))
}

pub(crate) fn to_i64<T>(value: T) -> Result<i64, TError>
where
    T: TryInto<i64> + Copy + Display,
{
    value
        .try_into()
        .map_err(|_| TError::InvalidQuantity(value.to_string()))
}

pub(crate) fn to_i32<T>(value: T) -> Result<i32, TError>
where
    T: TryInto<i32> + Copy + Display,
{
    value
        .try_into()
        .map_err(|_| TError::InvalidQuantity(value.to_string()))
}
