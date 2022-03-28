use chrono;
use tinkoff_invest_types as tit;

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    pub seconds: i64,
    pub nanoseconds: u32,
}

impl Into<DateTime> for chrono::DateTime<chrono::offset::Utc> {
    fn into(self) -> DateTime {
        DateTime {
            seconds: self.timestamp(),
            nanoseconds: self.timestamp_subsec_nanos(),
        }
    }
}

impl Into<DateTime> for tit::extra::Timestamp {
    fn into(self) -> DateTime {
        DateTime {
            seconds: self.seconds,
            nanoseconds: self.nanos as u32,
        }
    }
}

impl Into<DateTime> for &tit::extra::Timestamp {
    fn into(self) -> DateTime {
        DateTime {
            seconds: self.seconds,
            nanoseconds: self.nanos as u32,
        }
    }
}

// impl Into<chrono::DateTime<chrono::offset::Utc>> for DateTime {
//     fn into(self) -> chrono::DateTime<chrono::offset::Utc> {
//         self.chrono
//     }
// }

impl Into<tit::extra::Timestamp> for DateTime {
    fn into(self) -> tinkoff_invest_types::extra::Timestamp {
        tinkoff_invest_types::extra::Timestamp {
            seconds: self.seconds,
            nanos: self.nanoseconds as i32,
        }
    }
}
