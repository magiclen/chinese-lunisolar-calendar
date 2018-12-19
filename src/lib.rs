#[macro_use]
extern crate lazy_static;

extern crate chinese_variant;

pub use chinese_variant::ChineseVariant;

pub extern crate chrono;

use chrono::prelude::*;

mod constants;

pub(crate) use constants::*;

lazy_static! {
    /// 最大西(陽)曆日：2101-01-18。*(lazy_static)*
    pub static ref MAX_DATE_IN_SOLAR_CALENDAR: NaiveDate = Utc.ymd(2101, 1, 18).naive_utc();

    /// 最小西(陽)曆日：1901-02-19。*(lazy_static)*
    pub static ref MIN_DATE_IN_SOLAR_CALENDAR: NaiveDate = Utc.ymd(1901, 2, 19).naive_utc();
}

mod zodiac;

pub use self::zodiac::Zodiac;

mod earthly_branch;

pub use self::earthly_branch::EarthlyBranch;

mod heavenly_stems;

pub use self::heavenly_stems::HeavenlyStems;

mod lunar_year;

pub use self::lunar_year::LunarYear;

mod lunar_month;

pub use self::lunar_month::LunarMonth;

mod lunar_day;

pub use self::lunar_day::LunarDay;

mod lunar_date;

pub use self::lunar_date::LunarDate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
