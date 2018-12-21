#[macro_use]
extern crate lazy_static;

extern crate chinese_variant;

pub use chinese_variant::ChineseVariant;

pub extern crate chrono;

use chrono::prelude::*;

mod constants;

pub(crate) use constants::*;

/// 最大支援的農曆西曆年。*(u16)*
pub const MAX_YEAR_IN_SOLAR_CALENDAR: u16 = 2101;

/// 最小支援的農曆西曆年。*(u16)*
pub const MIN_YEAR_IN_SOLAR_CALENDAR: u16 = 1901;

lazy_static! {
    /// 最大支援的農曆日期(以西曆日期表示)：2101-01-18。*(lazy_static的實體)*
    pub static ref MAX_LUNAR_DATE_IN_SOLAR_CALENDAR: NaiveDate = Utc.ymd(MAX_YEAR_IN_SOLAR_CALENDAR as i32, 1, 28).naive_utc();

    /// 最大支援的農曆日期的新年偏差。*(lazy_static的實體)*
    pub(crate) static ref MAX_LUNAR_DATE_IN_SOLAR_CALENDAR_NEW_YEAR_DIFFERENCE: u16 = SolarDate::from_naive_date(*MAX_LUNAR_DATE_IN_SOLAR_CALENDAR).unwrap().the_n_day_in_this_year();

    /// 最小支援的農曆日期(以西曆日期表示)：1901-02-19。*(lazy_static的實體)*
    pub static ref MIN_LUNAR_DATE_IN_SOLAR_CALENDAR: NaiveDate = Utc.ymd(MIN_YEAR_IN_SOLAR_CALENDAR as i32, 2, 19).naive_utc();
}

mod lunisolar_error;

pub use self::lunisolar_error::LunisolarError;

mod zodiac;

pub use self::zodiac::Zodiac;

mod earthly_branch;

pub use self::earthly_branch::EarthlyBranch;

mod heavenly_stems;

pub use self::heavenly_stems::HeavenlyStems;

mod solar_year;

pub use self::solar_year::SolarYear;

mod solar_month;

pub use self::solar_month::SolarMonth;

mod solar_day;

pub use self::solar_day::SolarDay;

mod solar_date;

pub use self::solar_date::SolarDate;

mod lunar_year;

pub use self::lunar_year::LunarYear;

mod lunar_month;

pub use self::lunar_month::LunarMonth;

mod lunar_day;

pub use self::lunar_day::LunarDay;

mod lunar_date;

pub use self::lunar_date::LunarDate;

mod lunisolar_year;

pub use self::lunisolar_year::LunisolarYear;