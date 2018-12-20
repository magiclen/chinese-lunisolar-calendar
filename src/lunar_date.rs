use super::{ChineseVariant, SolarYear, LunarYear, LunarMonth, LunarDay, MIN_LUNAR_DATE_IN_SOLAR_CALENDAR, MAX_LUNAR_DATE_IN_SOLAR_CALENDAR};

use chrono::prelude::*;

use chrono::NaiveDate;

/// 農曆日期，必須包含西曆年。
#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunarDate {
    pub(crate) solar_year: SolarYear,
    pub(crate) lunar_year: LunarYear,
    pub(crate) lunar_month: LunarMonth,
    pub(crate) lunar_day: LunarDay,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LunarDateParseError {
    /// 超出支援的日期範圍。
    OutOfRange
}

impl LunarDate {
    pub fn from_naive_date<>(naive_date: NaiveDate) -> Result<LunarDate, LunarDateParseError> {
        if naive_date < *MIN_LUNAR_DATE_IN_SOLAR_CALENDAR || naive_date > *MAX_LUNAR_DATE_IN_SOLAR_CALENDAR {
            Err(LunarDateParseError::OutOfRange)
        } else {
            let solar_year = SolarYear::from_u16(naive_date.year() as u16);
            Err(LunarDateParseError::OutOfRange)
        }
    }
}