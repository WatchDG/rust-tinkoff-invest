use std::convert::TryFrom;
use uuid::Uuid;

use crate::TError;
use crate::traits;

/// Уникальный идентификатор инструмента (UUID).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uid(Uuid);

impl Uid {
    /// Создаёт UID из уже распарсенного UUID.
    #[inline]
    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Возвращает внутренний UUID.
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Парсит UID из строки ответа API.
    ///
    /// При невалидном значении возвращает nil UUID (данные API считаются доверенными).
    #[inline]
    pub(crate) fn from_api_str(value: &str) -> Self {
        Self::try_from(value).unwrap_or_else(|_| Self(Uuid::nil()))
    }
}

impl TryFrom<&str> for Uid {
    type Error = TError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value)
            .map(Uid)
            .map_err(|e| TError::InvalidUid(format!("{value}: {e}")))
    }
}

impl TryFrom<String> for Uid {
    type Error = TError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uid::try_from(value.as_str())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_valid_uuid() {
        let uid = Uid::try_from("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(
            uid.as_uuid().to_string(),
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn try_from_invalid_uuid() {
        let err = Uid::try_from("not-a-uuid").unwrap_err();
        assert!(matches!(err, TError::InvalidUid(_)));
    }
}
