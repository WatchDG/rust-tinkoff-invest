use chrono;
use std::cmp::Ordering;
use tinkoff_invest_types as tit;

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

impl From<DateTime> for chrono::NaiveDateTime {
    fn from(value: DateTime) -> Self {
        chrono::DateTime::from_timestamp(value.seconds, value.nanoseconds)
            .unwrap()
            .naive_utc()
    }
}

impl From<chrono::NaiveDate> for DateTime {
    fn from(value: chrono::NaiveDate) -> Self {
        let naive_date_time =
            chrono::NaiveDateTime::new(value, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        naive_date_time.into()
    }
}

impl From<tit::prost_types::Timestamp> for DateTime {
    fn from(value: tit::prost_types::Timestamp) -> Self {
        DateTime {
            seconds: value.seconds,
            nanoseconds: value.nanos as u32,
        }
    }
}

impl From<DateTime> for tit::prost_types::Timestamp {
    fn from(value: DateTime) -> Self {
        tit::prost_types::Timestamp {
            seconds: value.seconds,
            nanos: value.nanoseconds as i32,
        }
    }
}

impl DateTime {
    pub fn now() -> Self {
        chrono::Utc::now().naive_utc().into()
    }
}
