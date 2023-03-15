use super::THE_LUNAR_DAYS;

#[cfg(feature = "ba-zi-weight")]
use super::BA_ZI_WEIGHT_DAYS;

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// 列舉農曆三十個天數名稱：初一、初二、...、十一、十二、...、廿一、廿二、...、三十。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub enum LunarDay {
    /// 初一
    First,
    /// 初二
    Second,
    /// 初三
    Third,
    /// 初四
    Fourth,
    /// 初五
    Fifth,
    /// 初六
    Sixth,
    /// 初七
    Seventh,
    /// 初八
    Eighth,
    /// 初九
    Ninth,
    /// 初十
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
    /// 廿一
    TwentyFirst,
    /// 廿二
    TwentySecond,
    /// 廿三
    TwentyThird,
    /// 廿四
    TwentyFourth,
    /// 廿五
    TwentyFifth,
    /// 廿六
    TwentySixth,
    /// 廿七
    TwentySeventh,
    /// 廿八
    TwentyEighth,
    /// 廿九
    TwentyNinth,
    /// 三十
    Thirty,
}

impl LunarDay {
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_ordinal_unsafe(number: i8) -> LunarDay {
        transmute(number)
    }

    /// 透過農曆日期字串來取得 `LunarDay` 列舉實體。
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarDay> {
        let s = s.as_ref();

        for (i, &t) in THE_LUNAR_DAYS.iter().enumerate() {
            if t == s {
                return Some(unsafe { Self::from_ordinal_unsafe(i as i8) });
            }
        }

        None
    }

    /// 取得 `LunarDay` 列舉實體所代表的農曆日期字串。
    #[inline]
    pub fn to_str(self) -> &'static str {
        let i = self as usize;

        THE_LUNAR_DAYS[i]
    }

    /// 透過農曆日期數值來取得 `LunarDay` 列舉實體。
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_u8_unsafe(day: u8) -> LunarDay {
        transmute(day - 1)
    }

    /// 透過農曆日期數值來取得 `LunarDay` 列舉實體。
    #[inline]
    pub fn from_u8(day: u8) -> Option<LunarDay> {
        if day == 0 || day > 30 {
            None
        } else {
            Some(unsafe { Self::from_u8_unsafe(day) })
        }
    }

    /// 取得 `LunarDay` 列舉實體所代表的農曆日期數值。
    #[inline]
    pub fn to_u8(self) -> u8 {
        self as u8 + 1
    }

    #[cfg(feature = "ba-zi-weight")]
    /// 取得八字重量。
    #[inline]
    pub fn get_ba_zi_weight(self) -> u8 {
        let i = self as usize;

        BA_ZI_WEIGHT_DAYS[i]
    }
}

impl Display for LunarDay {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}

impl FromStr for LunarDay {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LunarDay::from_str(s).ok_or(())
    }
}
