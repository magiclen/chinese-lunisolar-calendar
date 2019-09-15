use super::THE_SOLAR_DAYS;

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// 列舉西曆三十一個天數名稱：一、二、...、十一、十二、...、二十一、二十二、...、三十、三十一。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub enum SolarDay {
    /// 一
    First,
    /// 二
    Second,
    /// 三
    Third,
    /// 四
    Fourth,
    /// 五
    Fifth,
    /// 六
    Sixth,
    /// 七
    Seventh,
    /// 八
    Eighth,
    /// 九
    Ninth,
    /// 十
    Tenth,
    /// 十一
    Eleventh,
    /// 十二
    Twelfth,
    /// 十三
    Thirteen,
    /// 十四
    Fourteen,
    /// 十五
    Fifteen,
    /// 十六
    Sixteen,
    /// 十七
    Seventeen,
    /// 十八
    Eighteen,
    /// 十九
    Nineteen,
    /// 二十
    Twenty,
    /// 二十一
    TwentyFirst,
    /// 二十二
    TwentySecond,
    /// 二十三
    TwentyThird,
    /// 二十四
    TwentyFourth,
    /// 二十五
    TwentyFifth,
    /// 二十六
    TwentySixth,
    /// 二十七
    TwentySeventh,
    /// 二十八
    TwentyEighth,
    /// 二十九
    TwentyNinth,
    /// 三十
    Thirty,
    /// 三十一
    ThirtyFirst,
}

impl SolarDay {
    pub unsafe fn from_ordinal_unsafe(number: i8) -> SolarDay {
        transmute(number)
    }

    /// 透過西曆日期字串來取得 `SolarDay` 列舉實體。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<SolarDay> {
        let s = s.as_ref();

        for (i, &t) in THE_SOLAR_DAYS.iter().enumerate() {
            if t.eq(s) {
                return Some(unsafe { Self::from_ordinal_unsafe(i as i8) });
            }
        }

        None
    }

    /// 取得 `SolarDay` 列舉實體所代表的西曆日期字串。
    pub fn to_str(self) -> &'static str {
        let i = self as usize;

        THE_SOLAR_DAYS[i]
    }

    /// 透過西曆日期數值來取得 `SolarDay` 列舉實體。
    pub unsafe fn from_u8_unsafe(day: u8) -> SolarDay {
        transmute(day - 1)
    }

    /// 透過西曆日期數值來取得 `SolarDay` 列舉實體。
    pub fn from_u8(day: u8) -> Option<SolarDay> {
        if day == 0 || day > 31 {
            None
        } else {
            Some(unsafe { Self::from_u8_unsafe(day) })
        }
    }

    /// 取得 `SolarDay` 列舉實體所代表的西曆日期數值。
    pub fn to_u8(self) -> u8 {
        self as u8 + 1
    }
}

impl Display for SolarDay {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}

impl FromStr for SolarDay {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SolarDay::from_str(s).ok_or(())
    }
}
