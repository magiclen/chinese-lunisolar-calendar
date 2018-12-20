use super::THE_SOLAR_MONTHS;

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};

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

macro_rules! the_solar_months_from {
    ($a:expr, $v:expr) => {
        if $a[0].eq($v) {
            Some(SolarMonth::January)
        } else if $a[1].eq($v) {
            Some(SolarMonth::February)
        } else if $a[2].eq($v) {
            Some(SolarMonth::March)
        } else if $a[3].eq($v) {
            Some(SolarMonth::April)
        } else if $a[4].eq($v) {
            Some(SolarMonth::May)
        } else if $a[5].eq($v) {
            Some(SolarMonth::June)
        } else if $a[6].eq($v) {
            Some(SolarMonth::July)
        } else if $a[7].eq($v) {
            Some(SolarMonth::August)
        } else if $a[8].eq($v) {
            Some(SolarMonth::September)
        } else if $a[9].eq($v) {
            Some(SolarMonth::October)
        } else if $a[10].eq($v) {
            Some(SolarMonth::November)
        } else if $a[11].eq($v) {
            Some(SolarMonth::December)
        } else {
            None
        }
    };
}

impl SolarMonth {
    /// 透過西曆月份字串來取得 `SolarMonth` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<SolarMonth> {
        let s = s.as_ref();

        the_solar_months_from!(THE_SOLAR_MONTHS, s)
    }

    /// 取得 `SolarMonth` 列舉實體所代表的西曆月份字串。
    pub fn to_str(&self) -> &'static str {
        let i = *self as usize;

        THE_SOLAR_MONTHS[i]
    }

    /// 透過西曆月份數值來取得 `SolarMonth` 列舉實體。
    pub unsafe fn from_u8_unsafe(month: u8) -> SolarMonth {
        transmute(month - 1)
    }

    /// 透過西曆月份數值來取得 `SolarMonth` 列舉實體。
    pub fn from_u8(month: u8) -> Option<SolarMonth> {
        if month == 0 || month > 12 {
            None
        } else {
            Some(unsafe {
                Self::from_u8_unsafe(month)
            })
        }
    }

    /// 取得 `SolarMonth` 列舉實體所代表的西曆月份數值。
    pub fn to_u8(&self) -> u8 {
        let i = *self as u8;

        i + 1
    }
}

impl Display for SolarMonth {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}