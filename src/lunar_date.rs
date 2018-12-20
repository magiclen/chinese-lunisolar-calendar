use super::{ChineseVariant, LunisolarError, LunisolarYear, SolarDate, LunarYear, LunarMonth, LunarDay, MIN_LUNAR_DATE_IN_SOLAR_CALENDAR, MAX_LUNAR_DATE_IN_SOLAR_CALENDAR};

use chrono::prelude::*;

use chrono::NaiveDate;

/// 農曆日期，必須包含西曆年。
#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunarDate {
    lunisolar_year: LunisolarYear,
    lunar_year: LunarYear,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
}

impl LunarDate {
    fn from_naive_date_inner(solar_date: SolarDate, naive_date: NaiveDate) -> Result<LunarDate, LunisolarError> {
        let solar_year = solar_date.get_solar_year();

        let new_year_diff = solar_date.the_n_day_in_this_year() - 1;


        Err(LunisolarError::OutOfLunarRange)
    }

    pub fn from_naive_date(naive_date: NaiveDate) -> Result<LunarDate, LunisolarError> {
        if naive_date < *MIN_LUNAR_DATE_IN_SOLAR_CALENDAR || naive_date > *MAX_LUNAR_DATE_IN_SOLAR_CALENDAR {
            Err(LunisolarError::OutOfLunarRange)
        } else {
            let solar_date = SolarDate::from_naive_date(naive_date)?;

            Self::from_naive_date_inner(solar_date, naive_date)
        }
    }

    pub fn from_date<Tz: TimeZone>(date: Date<Tz>) -> Result<LunarDate, LunisolarError> {
        let naive_date = date.naive_utc();

        Self::from_naive_date(naive_date)
    }
}