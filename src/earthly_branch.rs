use super::{THE_EARTHLY_BRANCHES, THE_EARTHLY_BRANCHES_CHARS, Zodiac};

use std::mem::transmute;

/// 列舉中國十二地支：子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub enum EarthlyBranch {
    /// 子
    First,
    /// 丑
    Second,
    /// 寅
    Third,
    /// 卯
    Fourth,
    /// 辰
    Fifth,
    /// 巳
    Sixth,
    /// 午
    Seventh,
    /// 未
    Eighth,
    /// 申
    Ninth,
    /// 酉
    Tenth,
    /// 戌
    Eleventh,
    /// 亥
    Twelfth,
}

macro_rules! the_earthly_branches_from {
    ($a:expr, $v:expr) => {
        if $a[0].eq($v) {
            Some(EarthlyBranch::First)
        } else if $a[1].eq($v) {
            Some(EarthlyBranch::Second)
        } else if $a[2].eq($v) {
            Some(EarthlyBranch::Third)
        } else if $a[3].eq($v) {
            Some(EarthlyBranch::Fourth)
        } else if $a[4].eq($v) {
            Some(EarthlyBranch::Fifth)
        } else if $a[5].eq($v) {
            Some(EarthlyBranch::Sixth)
        } else if $a[6].eq($v) {
            Some(EarthlyBranch::Seventh)
        } else if $a[7].eq($v) {
            Some(EarthlyBranch::Eighth)
        } else if $a[8].eq($v) {
            Some(EarthlyBranch::Ninth)
        } else if $a[9].eq($v) {
            Some(EarthlyBranch::Tenth)
        } else if $a[10].eq($v) {
            Some(EarthlyBranch::Eleventh)
        } else if $a[11].eq($v) {
            Some(EarthlyBranch::Twelfth)
        } else {
            None
        }
    };
}

impl EarthlyBranch {
    /// 透過子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥等字串來取得 `EarthlyBranch` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<EarthlyBranch> {
        let s = s.as_ref();

        the_earthly_branches_from!(THE_EARTHLY_BRANCHES, s)
    }

    /// 取得 `EarthlyBranch` 列舉實體所代表的地支字串。
    pub fn to_str(&self) -> &'static str {
        let i = *self as usize;

        THE_EARTHLY_BRANCHES[i]
    }

    /// 透過子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥等字元來取得 `EarthlyBranch` 列舉實體。
    pub fn from_char(c: char) -> Option<EarthlyBranch> {
        the_earthly_branches_from!(THE_EARTHLY_BRANCHES_CHARS, &c)
    }

    /// 取得 `EarthlyBranch` 列舉實體所代表的地支字元。
    pub fn to_char(&self) -> char {
        let i = *self as usize;

        THE_EARTHLY_BRANCHES_CHARS[i]
    }

    /// 透過生肖來取得地支。
    pub fn from_zodiac(zodiac: Zodiac) -> EarthlyBranch {
        unsafe { transmute(zodiac) }
    }

    /// 將地支轉成生肖。
    pub fn to_zodiac(&self) -> Zodiac {
        unsafe { transmute(*self) }
    }
}