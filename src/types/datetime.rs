use chrono;
use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime {
    pub seconds: i64,
    pub nanoseconds: u32,
}

impl From<chrono::NaiveDateTime> for DateTime {
    fn from(value: chrono::NaiveDateTime) -> Self {
        DateTime {
            seconds: value.timestamp(),
            nanoseconds: value.timestamp_subsec_nanos(),
        }
    }
}

impl From<DateTime> for chrono::NaiveDateTime {
    fn from(value: DateTime) -> Self {
        chrono::NaiveDateTime::from_timestamp(value.seconds, value.nanoseconds)
    }
}

impl From<chrono::NaiveDate> for DateTime {
    fn from(value: chrono::NaiveDate) -> Self {
        let naive_date_time =
            chrono::NaiveDateTime::new(value, chrono::NaiveTime::from_hms(0, 0, 0));
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
