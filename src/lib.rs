/*!
# Chinese Lunisolar Calendar

The traditional Chinese Calendar, called **農曆** or **陰曆** in Chinese, is based on the moon, so it's also known as Lunar Calendar.

Because the cycle of Lunar Calendar is 60 years and there are no regular rules for the days in each lunar month and even the number of months in a lunar year, it's hard to use Lunar Calendar without referencing any other calendars. The Lunisolar Calendar is a way to combine both of the Solar Calendar (Gregorian Calendar) and the Lunar Calendar in order to make it accurate, predictable and useful.

The Lunisolar Calendar relies on three datasets, `BIG_MONTHS`, `LEAP_MONTHS`, and `NEW_YEAR_DIFFERENCE` which are written in the **constants.rs** file. Currently, the data range is from 1901 to 2100 (lunisolar year), so this Lunisolar Calendar supports from 1901-02-19 to 2101-01-28 (in Solar Calendar).

This library allows you to convert a date between the Lunisolar Calendar and the Solar Calendar, and to compute the weight of Ba Zi(八字). Moreover, it can convert a date to a Chinese text string and parse a Chinese text string to a date in Simple Chinese or Traditional Chinese.

## Examples

```rust
# #![allow(deprecated)] // wait for chrono 0.5

use chinese_lunisolar_calendar::chrono::prelude::*;

use chinese_lunisolar_calendar::SolarDate;

let solar_date = SolarDate::from_naive_date(NaiveDate::from_ymd(2019, 1, 15)).unwrap();

assert_eq!("二零一九年一月十五日", solar_date.to_chinese_string());
assert_eq!("二〇一九年一月十五日", solar_date.to_chinese_string_2());
```

```rust
# #![allow(deprecated)] // wait for chrono 0.5

use chinese_lunisolar_calendar::chrono::prelude::*;

use chinese_lunisolar_calendar::{ChineseVariant, LunisolarDate};

let lunisolar_date = LunisolarDate::from_naive_date(NaiveDate::from_ymd(2019, 1, 15)).unwrap();

assert_eq!(2019, lunisolar_date.get_solar_year().to_u16());
assert_eq!("二零一八　戊戌、狗年　臘月　初十", lunisolar_date.to_chinese_string(ChineseVariant::Traditional));
assert_eq!("二零一八　戊戌、狗年　腊月　初十", lunisolar_date.to_chinese_string(ChineseVariant::Simple));
assert_eq!("二〇一八　戊戌、狗年　臘月　初十", lunisolar_date.to_chinese_string_2(ChineseVariant::Traditional));
assert_eq!("二〇一八　戊戌、狗年　腊月　初十", lunisolar_date.to_chinese_string_2(ChineseVariant::Simple));

assert_eq!(4.3, lunisolar_date.get_ba_zi_weight_by_time(NaiveTime::from_hms(15, 30, 0)));
```
*/

#![allow(deprecated)] // wait for chrono 0.5

pub extern crate chrono;

pub use chinese_variant::ChineseVariant;

use once_cell::sync::Lazy;

use chrono::prelude::*;

mod constants;

pub(crate) use constants::*;

/// 最大支援的農曆西曆年。*(u16)*
pub const MAX_YEAR_IN_SOLAR_CALENDAR: u16 = 2101;

/// 最小支援的農曆西曆年。*(u16)*
pub const MIN_YEAR_IN_SOLAR_CALENDAR: u16 = 1901;

/// 最大支援的農曆日期(以西曆日期表示)：2101-01-28。*(`once_cell::sync::Lazy` 的實體)*
pub static MAX_LUNAR_DATE_IN_SOLAR_CALENDAR: Lazy<NaiveDate> =
    Lazy::new(|| Utc.ymd(i32::from(MAX_YEAR_IN_SOLAR_CALENDAR), 1, 28).naive_utc());

pub(crate) static MAX_LUNAR_DATE_IN_SOLAR_CALENDAR_NEW_YEAR_DIFFERENCE: Lazy<u16> =
    Lazy::new(|| {
        SolarDate::from_naive_date(*MAX_LUNAR_DATE_IN_SOLAR_CALENDAR)
            .unwrap()
            .the_n_day_in_this_year()
    });

/// 最小支援的農曆日期(以西曆日期表示)：1901-02-19。*(`once_cell::sync::Lazy` 的實體)*
pub static MIN_LUNAR_DATE_IN_SOLAR_CALENDAR: Lazy<NaiveDate> =
    Lazy::new(|| Utc.ymd(i32::from(MIN_YEAR_IN_SOLAR_CALENDAR), 2, 19).naive_utc());

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
