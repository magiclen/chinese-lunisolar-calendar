use super::{THE_LUNAR_MONTHS, ChineseVariant};

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};

/// 列舉農曆十二個月份名稱：正月、二月、三月、四月、五月、六月、七月、八月、九月、十月、冬月、臘月。包含閏月。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub enum LunarMonth {
    /// 正月
    First,
    /// 閏正月
    LeapFirst,
    /// 二月
    Second,
    /// 閏二月
    LeapSecond,
    /// 三月
    Third,
    /// 閏三月
    LeapThird,
    /// 四月
    Fourth,
    /// 閏四月
    LeapFourth,
    /// 五月
    Fifth,
    /// 閏五月
    LeapFifth,
    /// 六月
    Sixth,
    /// 閏六月
    LeapSixth,
    /// 七月
    Seventh,
    /// 閏七月
    LeapSeventh,
    /// 八月
    Eighth,
    /// 閏八月
    LeapEighth,
    /// 九月
    Ninth,
    /// 閏九月
    LeapNinth,
    /// 十月
    Tenth,
    /// 閏十月
    LeapTenth,
    /// 冬月
    Eleventh,
    /// 閏冬月
    LeapEleventh,
    /// 臘月
    Twelfth,
    /// 閏臘月
    LeapTwelfth,
}

macro_rules! the_lunar_months_contains {
    ($a:expr, $v:expr) => {
        if $a[0].contains($v) || $a[24].contains($v) {
            Some(LunarMonth::First)
        } else if $a[1].contains($v) {
            Some(LunarMonth::Second)
        } else if $a[2].contains($v) {
            Some(LunarMonth::Third)
        } else if $a[3].contains($v) {
            Some(LunarMonth::Fourth)
        } else if $a[4].contains($v) {
            Some(LunarMonth::Fifth)
        } else if $a[5].contains($v) {
            Some(LunarMonth::Sixth)
        } else if $a[6].contains($v) {
            Some(LunarMonth::Seventh)
        } else if $a[7].contains($v) {
            Some(LunarMonth::Eighth)
        } else if $a[8].contains($v) {
            Some(LunarMonth::Ninth)
        } else if $a[9].contains($v) {
            Some(LunarMonth::Tenth)
        } else if $a[10].contains($v) || $a[25].contains($v) {
            Some(LunarMonth::Eleventh)
        } else if $a[11].contains($v) || $a[26].contains($v) {
            Some(LunarMonth::Twelfth)
        } else if $a[12].contains($v) || $a[27].contains($v) {
            Some(LunarMonth::LeapFirst)
        } else if $a[13].contains($v) {
            Some(LunarMonth::LeapSecond)
        } else if $a[14].contains($v) {
            Some(LunarMonth::LeapThird)
        } else if $a[15].contains($v) {
            Some(LunarMonth::LeapFourth)
        } else if $a[16].contains($v) {
            Some(LunarMonth::LeapFifth)
        } else if $a[17].contains($v) {
            Some(LunarMonth::LeapSixth)
        } else if $a[18].contains($v) {
            Some(LunarMonth::LeapSeventh)
        } else if $a[19].contains($v) {
            Some(LunarMonth::LeapEighth)
        } else if $a[20].contains($v) {
            Some(LunarMonth::LeapNinth)
        } else if $a[21].contains($v) {
            Some(LunarMonth::LeapTenth)
        } else if $a[22].contains($v) || $a[28].contains($v) {
            Some(LunarMonth::LeapEleventh)
        } else if $a[23].contains($v) || $a[29].contains($v) || $a[30].contains($v) {
            Some(LunarMonth::LeapTwelfth)
        } else {
            None
        }
    };
}

macro_rules! the_lunar_months_variants {
    ($a:expr, $v:expr, $i:expr) => {
        match $v {
            ChineseVariant::Simple => {
                $a[$i][1]
            }
            ChineseVariant::Traditional => {
                $a[$i][0]
            }
        }
    };
}

impl LunarMonth {
    /// 透過農曆月份字串來取得 `LunarMonth` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarMonth> {
        let s = s.as_ref();

        the_lunar_months_contains!(THE_LUNAR_MONTHS, &s)
    }

    /// 取得 `LunarMonth` 列舉實體所代表的農曆月份字串。
    pub fn to_str(&self, chinese_variant: ChineseVariant) -> &'static str {
        let mut i = *self as usize;

        if i % 2 == 1 {
            i = i / 2 + 12;
        } else {
            i = i / 2;
        }
        the_lunar_months_variants!(THE_LUNAR_MONTHS, chinese_variant, i)
    }

    /// 透過農曆月份數值和是否閏月來取得 `LunarMonth` 列舉實體。
    pub fn from_u8(month: u8, leap: bool) -> Option<LunarMonth> {
        if month == 0 || month > 12 {
            None
        } else {
            if leap {
                Some(unsafe {
                    transmute((month - 1) * 2 + 1)
                })
            } else {
                Some(unsafe {
                    transmute((month - 1) * 2)
                })
            }
        }
    }

    /// 取得 `LunarMonth` 列舉實體所代表的農曆月份數值。
    pub fn to_u8(&self) -> u8 {
        let i = *self as u8;

        i / 2 + 1
    }

    /// 是否為閏月。
    pub fn is_leap_month(&self) -> bool {
        let i = *self as usize;

        i % 2 == 1
    }
}

impl Display for LunarMonth {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str(ChineseVariant::Traditional))
    }
}