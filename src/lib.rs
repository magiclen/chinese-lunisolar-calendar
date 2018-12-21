/*!
# Chinese Linisolar Calendar

The traditional Chinese Calendar, called **農曆** or **陰曆** in Chinese, is based on the moon, so it's also known as Lunar Calendar.

Because the cycle of Lunar Calendar is 60 years and there are no regular rules for the days in each lunar month and even the number of months in a lunar year, it's hard to use Lunar Calendar without referencing any other calendars. The Linisolar Calendar is a way to combine both of the Solar Calendar (Gregorian Calendar) and the Lunar Calendar in order to make it accurate, predictable and useful.

The Linisolar Calendar relies on three datasets, `BIG_MONTHS`, `LEAP_MONTHS`, and `NEW_YEAR_DIFFERENCE` which are written in the **constants.rs** file. Currently, the data range is from 1901 to 2100 (linisolar year), so this Linisolar Calendar supports from 1901-02-19 to 2101-01-28 (in Solar Calendar).

This library allows you to convert a date between the Linisolar Calendar and the Solar Calendar, and to compute the weight of Ba Zi(八字). Moreover, it can convert a date to a Chinese text string and parse a Chinese text string to a date in Simple Chinese or Traditional Chinese.

## Examples

```rust
extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::chrono::prelude::*;

use chinese_lunisolar_calendar::SolarDate;

let solar_date = SolarDate::from_naive_date(NaiveDate::from_ymd(2019, 1, 15)).unwrap();

assert_eq!("二零一九年一月十五日", solar_date.to_chinese_string());
```

```rust
extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::chrono::prelude::*;

use chinese_lunisolar_calendar::{ChineseVariant, LunisolarDate};

let lunisolar_date = LunisolarDate::from_naive_date(NaiveDate::from_ymd(2019, 1, 15)).unwrap();

assert_eq!(2019, lunisolar_date.get_solar_year().to_u16());
assert_eq!("二零一八　戊戌、狗年　臘月　初十", lunisolar_date.to_chinese_string(ChineseVariant::Traditional));
assert_eq!("二零一八　戊戌、狗年　腊月　初十", lunisolar_date.to_chinese_string(ChineseVariant::Simple));

assert_eq!(4.3, lunisolar_date.get_ba_zi_weight_by_time(NaiveTime::from_hms(15, 30, 0)));
```
*/

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
    /// 最大支援的農曆日期(以西曆日期表示)：2101-01-28。*(lazy_static的實體)*
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

mod lunisolar_date;

pub use self::lunisolar_date::LunisolarDate;

mod lunisolar_year;

pub use self::lunisolar_year::LunisolarYear;