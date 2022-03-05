Chinese Lunisolar Calendar
====================

[![CI](https://github.com/magiclen/chinese-lunisolar-calendar/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/chinese-lunisolar-calendar/actions/workflows/ci.yml)

The traditional Chinese Calendar, called **農曆** or **陰曆** in Chinese, is based on the moon, so it's also known as Lunar Calendar.

Because the cycle of Lunar Calendar is 60 years and there are no regular rules for the days in each lunar month and even the number of months in a lunar year, it's hard to use Lunar Calendar without referencing any other calendars. The Lunisolar Calendar is a way to combine both of the Solar Calendar (Gregorian Calendar) and the Lunar Calendar in order to make it accurate, predictable and useful.

The Lunisolar Calendar relies on three datasets, `BIG_MONTHS`, `LEAP_MONTHS`, and `NEW_YEAR_DIFFERENCE` which are written in the **constants.rs** file. Currently, the data range is from 1901 to 2100 (lunisolar year), so this Lunisolar Calendar supports from 1901-02-19 to 2101-01-28 (in Solar Calendar).

This library allows you to convert a date between the Lunisolar Calendar and the Solar Calendar, and to compute the weight of Ba Zi(八字). Moreover, it can convert a date to a Chinese text string and parse a Chinese text string to a date in Simple Chinese or Traditional Chinese.

## Examples

```rust
use chinese_lunisolar_calendar::chrono::prelude::*;

use chinese_lunisolar_calendar::SolarDate;

let solar_date = SolarDate::from_naive_date(NaiveDate::from_ymd(2019, 1, 15)).unwrap();

assert_eq!("二零一九年一月十五日", solar_date.to_chinese_string());
assert_eq!("二〇一九年一月十五日", solar_date.to_chinese_string_2());
```

```rust
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

## Crates.io

https://crates.io/crates/chinese-lunisolar-calendar

## Documentation

https://docs.rs/chinese-lunisolar-calendar

## License

[MIT](LICENSE)