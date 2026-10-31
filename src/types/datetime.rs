use chrono;
use std::cmp::Ordering;
use std::convert::TryFrom;
use tinkoff_invest_types as tit;

use crate::TError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime {
    pub seconds: i64,
    pub nanoseconds: u32,
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.seconds > other.seconds {
            Some(Ordering::Greater)
        } else if self.seconds < other.seconds {
            Some(Ordering::Less)
        } else if self.nanoseconds > other.nanoseconds {
            Some(Ordering::Greater)
        } else if self.nanoseconds < other.nanoseconds {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl From<chrono::NaiveDateTime> for DateTime {
    fn from(value: chrono::NaiveDateTime) -> Self {
        DateTime {
            seconds: value.and_utc().timestamp(),
            nanoseconds: value.and_utc().timestamp_subsec_nanos(),
        }
    }
}

impl TryFrom<DateTime> for chrono::NaiveDateTime {
    type Error = TError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        chrono::DateTime::from_timestamp(value.seconds, value.nanoseconds)
            .map(|dt| dt.naive_utc())
            .ok_or_else(|| {
                TError::InvalidMetadata(format!(
                    "timestamp seconds={} nanos={}",
                    value.seconds, value.nanoseconds
                ))
            })
    }
}

impl From<chrono::NaiveDate> for DateTime {
    fn from(value: chrono::NaiveDate) -> Self {
        let naive_date_time =
            chrono::NaiveDateTime::new(value, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        naive_date_time.into()
    }
}

impl TryFrom<tit::prost_types::Timestamp> for DateTime {
    type Error = TError;

    fn try_from(value: tit::prost_types::Timestamp) -> Result<Self, Self::Error> {
        if !(0..1_000_000_000).contains(&value.nanos) {
            return Err(TError::InvalidMetadata(format!(
                "timestamp nanos {}",
                value.nanos
            )));
        }
        Ok(DateTime {
            seconds: value.seconds,
            nanoseconds: value.nanos as u32,
        })
    }
}

impl TryFrom<DateTime> for tit::prost_types::Timestamp {
    type Error = TError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        if value.nanoseconds >= 1_000_000_000 {
            return Err(TError::InvalidMetadata(format!(
                "timestamp nanos {}",
                value.nanoseconds
            )));
        }
        Ok(tit::prost_types::Timestamp {
            seconds: value.seconds,
            nanos: i32::try_from(value.nanoseconds).map_err(|_| {
                TError::InvalidMetadata(format!("timestamp nanos {}", value.nanoseconds))
            })?,
        })
    }
}

impl DateTime {
    pub fn now() -> Self {
        chrono::Utc::now().naive_utc().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_nanos_rejected() {
        let ts = tit::prost_types::Timestamp {
            seconds: 0,
            nanos: -1,
        };
        let err = DateTime::try_from(ts).unwrap_err();
        assert!(matches!(err, TError::InvalidMetadata(_)));
    }
}
