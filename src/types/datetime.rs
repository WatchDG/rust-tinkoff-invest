use chrono;
use std::cmp::Ordering;
use tinkoff_invest_types as tit;

/// Представляет дату и время в формате Unix timestamp с наносекундной точностью.
///
/// Структура хранит время как количество секунд с начала эпохи Unix (1 января 1970 года)
/// и дополнительно наносекунды для высокой точности.
///
/// # Примеры
///
/// ```
/// use tinkoff_invest::types::DateTime;
///
/// // Создание текущего времени
/// let now = DateTime::now();
///
/// // Создание из chrono::NaiveDateTime
/// use chrono::NaiveDateTime;
/// let chrono_dt = NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let dt = DateTime::from(chrono_dt);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime {
    /// Количество секунд с начала эпохи Unix (1 января 1970 года)
    pub seconds: i64,
    /// Количество наносекунд (0-999999999)
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
    /// Преобразует `chrono::NaiveDateTime` в `DateTime`.
    ///
    /// Время автоматически конвертируется в UTC и сохраняется как Unix timestamp.
    fn from(value: chrono::NaiveDateTime) -> Self {
        DateTime {
            seconds: value.and_utc().timestamp(),
            nanoseconds: value.and_utc().timestamp_subsec_nanos(),
        }
    }
}

impl From<DateTime> for chrono::NaiveDateTime {
    /// Преобразует `DateTime` в `chrono::NaiveDateTime`.
    ///
    /// Возвращает время в UTC без информации о часовом поясе.
    fn from(value: DateTime) -> Self {
        chrono::DateTime::from_timestamp(value.seconds, value.nanoseconds)
            .unwrap()
            .naive_utc()
    }
}

impl From<chrono::NaiveDate> for DateTime {
    /// Преобразует `chrono::NaiveDate` в `DateTime`.
    ///
    /// Время устанавливается на 00:00:00 указанной даты.
    fn from(value: chrono::NaiveDate) -> Self {
        let naive_date_time =
            chrono::NaiveDateTime::new(value, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        naive_date_time.into()
    }
}

impl From<tit::prost_types::Timestamp> for DateTime {
    /// Преобразует Protocol Buffers `Timestamp` в `DateTime`.
    ///
    /// Используется для совместимости с gRPC API Tinkoff Invest.
    fn from(value: tit::prost_types::Timestamp) -> Self {
        DateTime {
            seconds: value.seconds,
            nanoseconds: value.nanos as u32,
        }
    }
}

impl From<DateTime> for tit::prost_types::Timestamp {
    /// Преобразует `DateTime` в Protocol Buffers `Timestamp`.
    ///
    /// Используется для отправки данных в gRPC API Tinkoff Invest.
    fn from(value: DateTime) -> Self {
        tit::prost_types::Timestamp {
            seconds: value.seconds,
            nanos: value.nanoseconds as i32,
        }
    }
}

impl DateTime {
    /// Возвращает текущее время в UTC.
    ///
    /// # Примеры
    ///
    /// ```
    /// use tinkoff_invest::types::DateTime;
    ///
    /// let now = DateTime::now();
    /// println!("Текущее время: {} секунд, {} наносекунд", now.seconds, now.nanoseconds);
    /// ```
    pub fn now() -> Self {
        chrono::Utc::now().naive_utc().into()
    }
}

impl Default for DateTime {
    /// Возвращает текущее время как значение по умолчанию.
    ///
    /// Эквивалентно вызову `DateTime::now()`.
    fn default() -> Self {
        Self::now()
    }
}
