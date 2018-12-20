use super::THE_SOLAR_DAYS;

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};

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

macro_rules! the_solar_days_from {
    ($a:expr, $v:expr) => {
        if $a[0].eq($v) {
            Some(SolarDay::First)
        } else if $a[1].eq($v) {
            Some(SolarDay::Second)
        } else if $a[2].eq($v) {
            Some(SolarDay::Third)
        } else if $a[3].eq($v) {
            Some(SolarDay::Fourth)
        } else if $a[4].eq($v) {
            Some(SolarDay::Fifth)
        } else if $a[5].eq($v) {
            Some(SolarDay::Sixth)
        } else if $a[6].eq($v) {
            Some(SolarDay::Seventh)
        } else if $a[7].eq($v) {
            Some(SolarDay::Eighth)
        } else if $a[8].eq($v) {
            Some(SolarDay::Ninth)
        } else if $a[9].eq($v) {
            Some(SolarDay::Tenth)
        } else if $a[10].eq($v) {
            Some(SolarDay::Eleventh)
        } else if $a[11].eq($v) {
            Some(SolarDay::Twelfth)
        } else if $a[12].eq($v) {
            Some(SolarDay::Thirteen)
        } else if $a[13].eq($v) {
            Some(SolarDay::Fourteen)
        } else if $a[14].eq($v) {
            Some(SolarDay::Fifteen)
        } else if $a[15].eq($v) {
            Some(SolarDay::Sixteen)
        } else if $a[16].eq($v) {
            Some(SolarDay::Seventeen)
        } else if $a[17].eq($v) {
            Some(SolarDay::Eighteen)
        } else if $a[18].eq($v) {
            Some(SolarDay::Nineteen)
        } else if $a[19].eq($v) {
            Some(SolarDay::Twenty)
        } else if $a[20].eq($v) {
            Some(SolarDay::TwentyFirst)
        } else if $a[21].eq($v) {
            Some(SolarDay::TwentySecond)
        } else if $a[22].eq($v) {
            Some(SolarDay::TwentyThird)
        } else if $a[23].eq($v) {
            Some(SolarDay::TwentyFourth)
        } else if $a[24].eq($v) {
            Some(SolarDay::TwentyFifth)
        } else if $a[25].eq($v) {
            Some(SolarDay::TwentySixth)
        } else if $a[26].eq($v) {
            Some(SolarDay::TwentySeventh)
        } else if $a[27].eq($v) {
            Some(SolarDay::TwentyEighth)
        } else if $a[28].eq($v) {
            Some(SolarDay::TwentyNinth)
        } else if $a[29].eq($v) {
            Some(SolarDay::Thirty)
        } else if $a[30].eq($v) {
            Some(SolarDay::ThirtyFirst)
        } else {
            None
        }
    };
}

impl SolarDay {
    /// 透過西曆日期字串來取得 `SolarDay` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<SolarDay> {
        let s = s.as_ref();

        the_solar_days_from!(THE_SOLAR_DAYS, s)
    }

    /// 取得 `SolarDay` 列舉實體所代表的西曆日期字串。
    pub fn to_str(&self) -> &'static str {
        let i = *self as usize;

        THE_SOLAR_DAYS[i]
    }

    /// 透過西曆日期數值來取得 `SolarDay` 列舉實體。
    pub fn from_u8(day: u8) -> Option<SolarDay> {
        if day == 0 || day > 31 {
            None
        } else {
            Some(unsafe {
                transmute(day - 1)
            })
        }
    }

    /// 取得 `SolarDay` 列舉實體所代表的西曆日期數值。
    pub fn to_u8(&self) -> u8 {
        *self as u8 + 1
    }
}

impl Display for SolarDay {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}