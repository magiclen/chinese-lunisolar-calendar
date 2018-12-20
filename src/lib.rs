#[macro_use]
extern crate lazy_static;

extern crate chinese_variant;

pub use chinese_variant::ChineseVariant;

pub extern crate chrono;

use chrono::prelude::*;

mod constants;

pub(crate) use constants::*;

lazy_static! {
    /// 最大支援的農曆日期(以西曆日期表示)：2101-01-18。*(lazy_static的實體)*
    pub static ref MAX_LUNAR_DATE_IN_SOLAR_CALENDAR: NaiveDate = Utc.ymd(2101, 1, 18).naive_utc();

    /// 最小支援的農曆日期(以西曆日期表示)：1901-02-19。*(lazy_static的實體)*
    pub static ref MIN_LUNAR_DATE_IN_SOLAR_CALENDAR: NaiveDate = Utc.ymd(1901, 2, 19).naive_utc();
}

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

pub use self::solar_date::{SolarDate, SolarDateParseError};

mod lunar_year;

pub use self::lunar_year::LunarYear;

mod lunar_month;

pub use self::lunar_month::LunarMonth;

mod lunar_day;

pub use self::lunar_day::LunarDay;

mod lunar_date;

pub use self::lunar_date::{LunarDate, LunarDateParseError};

mod functions;

pub use self::functions::*;