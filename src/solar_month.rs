use super::{SolarYear, THE_SOLAR_MONTHS};

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// 列舉西曆十二個月份名稱：一月、二月、三月、四月、五月、六月、七月、八月、九月、十月、十一月、十二月。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub enum SolarMonth {
    /// 一月
    January,
    /// 二月
    February,
    /// 三月
    March,
    /// 四月
    April,
    /// 五月
    May,
    /// 六月
    June,
    /// 七月
    July,
    /// 八月
    August,
    /// 九月
    September,
    /// 十月
    October,
    /// 十一月
    November,
    /// 十二月
    December,
}

impl SolarMonth {
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_ordinal_unsafe(number: i8) -> SolarMonth {
        transmute(number)
    }

    /// 透過西曆月份字串來取得 `SolarMonth` 列舉實體。
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<SolarMonth> {
        let s = s.as_ref();

        for (i, &t) in THE_SOLAR_MONTHS.iter().enumerate() {
            if t == s {
                return Some(unsafe { Self::from_ordinal_unsafe(i as i8) });
            }
        }

        None
    }

    /// 取得 `SolarMonth` 列舉實體所代表的西曆月份字串。
    #[inline]
    pub fn to_str(self) -> &'static str {
        let i = self as usize;

        THE_SOLAR_MONTHS[i]
    }

    /// 透過西曆月份數值來取得 `SolarMonth` 列舉實體。
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_u8_unsafe(month: u8) -> SolarMonth {
        transmute(month - 1)
    }

    /// 透過西曆月份數值來取得 `SolarMonth` 列舉實體。
    #[inline]
    pub fn from_u8(month: u8) -> Option<SolarMonth> {
        if month == 0 || month > 12 {
            None
        } else {
            Some(unsafe { Self::from_u8_unsafe(month) })
        }
    }

    /// 取得 `SolarMonth` 列舉實體所代表的西曆月份數值。
    #[inline]
    pub fn to_u8(self) -> u8 {
        let i = self as u8;

        i + 1
    }

    /// 傳入指定的西曆年，並計算此西曆月在這個指定的西曆年內共有幾天。
    #[inline]
    pub fn get_total_days<Y: Into<SolarYear>>(self, solar_year: Y) -> u8 {
        let month = self.to_u8();

        let m = u8::from(month < 8);

        if month % 2 == m {
            31
        } else if month == 2 {
            if solar_year.into().is_leap() {
                29
            } else {
                28
            }
        } else {
            30
        }
    }
}

impl Display for SolarMonth {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}

impl FromStr for SolarMonth {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SolarMonth::from_str(s).ok_or(())
    }
}
