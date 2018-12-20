use super::THE_LUNAR_DAYS;

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};

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

macro_rules! the_lunar_days_from {
    ($a:expr, $v:expr) => {
        if $a[0].eq($v) {
            Some(LunarDay::First)
        } else if $a[1].eq($v) {
            Some(LunarDay::Second)
        } else if $a[2].eq($v) {
            Some(LunarDay::Third)
        } else if $a[3].eq($v) {
            Some(LunarDay::Fourth)
        } else if $a[4].eq($v) {
            Some(LunarDay::Fifth)
        } else if $a[5].eq($v) {
            Some(LunarDay::Sixth)
        } else if $a[6].eq($v) {
            Some(LunarDay::Seventh)
        } else if $a[7].eq($v) {
            Some(LunarDay::Eighth)
        } else if $a[8].eq($v) {
            Some(LunarDay::Ninth)
        } else if $a[9].eq($v) {
            Some(LunarDay::Tenth)
        } else if $a[10].eq($v) {
            Some(LunarDay::Eleventh)
        } else if $a[11].eq($v) {
            Some(LunarDay::Twelfth)
        } else if $a[12].eq($v) {
            Some(LunarDay::Thirteen)
        } else if $a[13].eq($v) {
            Some(LunarDay::Fourteen)
        } else if $a[14].eq($v) {
            Some(LunarDay::Fifteen)
        } else if $a[15].eq($v) {
            Some(LunarDay::Sixteen)
        } else if $a[16].eq($v) {
            Some(LunarDay::Seventeen)
        } else if $a[17].eq($v) {
            Some(LunarDay::Eighteen)
        } else if $a[18].eq($v) {
            Some(LunarDay::Nineteen)
        } else if $a[19].eq($v) {
            Some(LunarDay::Twenty)
        } else if $a[20].eq($v) {
            Some(LunarDay::TwentyFirst)
        } else if $a[21].eq($v) {
            Some(LunarDay::TwentySecond)
        } else if $a[22].eq($v) {
            Some(LunarDay::TwentyThird)
        } else if $a[23].eq($v) {
            Some(LunarDay::TwentyFourth)
        } else if $a[24].eq($v) {
            Some(LunarDay::TwentyFifth)
        } else if $a[25].eq($v) {
            Some(LunarDay::TwentySixth)
        } else if $a[26].eq($v) {
            Some(LunarDay::TwentySeventh)
        } else if $a[27].eq($v) {
            Some(LunarDay::TwentyEighth)
        } else if $a[28].eq($v) {
            Some(LunarDay::TwentyNinth)
        } else if $a[29].eq($v) {
            Some(LunarDay::Thirty)
        } else {
            None
        }
    };
}

impl LunarDay {
    /// 透過農曆日期字串來取得 `LunarDay` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarDay> {
        let s = s.as_ref();

        the_lunar_days_from!(THE_LUNAR_DAYS, s)
    }

    /// 取得 `LunarDay` 列舉實體所代表的農曆日期字串。
    pub fn to_str(&self) -> &'static str {
        let i = *self as usize;

        THE_LUNAR_DAYS[i]
    }

    /// 透過農曆日期數值來取得 `LunarDay` 列舉實體。
    pub fn from_u8(day: u8) -> Option<LunarDay> {
        if day == 0 || day > 30 {
            None
        } else {
            Some(unsafe {
                transmute(day - 1)
            })
        }
    }

    /// 取得 `LunarDay` 列舉實體所代表的農曆日期數值。
    pub fn to_u8(&self) -> u8 {
        *self as u8 + 1
    }
}

impl Display for LunarDay {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}