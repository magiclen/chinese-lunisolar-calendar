use super::{THE_HEAVENLY_STEMS, THE_HEAVENLY_STEMS_CHARS};

use std::fmt::{self, Display, Formatter};

/// 列舉中國十天干：甲、乙、丙、丁、戊、己、更、辛、壬、葵。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub enum HeavenlyStems {
    /// 甲
    First,
    /// 乙
    Second,
    /// 丙
    Third,
    /// 丁
    Fourth,
    /// 戊
    Fifth,
    /// 己
    Sixth,
    /// 更
    Seventh,
    /// 辛
    Eighth,
    /// 壬
    Ninth,
    /// 葵
    Tenth,
}

macro_rules! the_heavenly_stems_from {
    ($a:expr, $v:expr) => {
        if $a[0].eq($v) {
            Some(HeavenlyStems::First)
        } else if $a[1].eq($v) {
            Some(HeavenlyStems::Second)
        } else if $a[2].eq($v) {
            Some(HeavenlyStems::Third)
        } else if $a[3].eq($v) {
            Some(HeavenlyStems::Fourth)
        } else if $a[4].eq($v) {
            Some(HeavenlyStems::Fifth)
        } else if $a[5].eq($v) {
            Some(HeavenlyStems::Sixth)
        } else if $a[6].eq($v) {
            Some(HeavenlyStems::Seventh)
        } else if $a[7].eq($v) {
            Some(HeavenlyStems::Eighth)
        } else if $a[8].eq($v) {
            Some(HeavenlyStems::Ninth)
        } else if $a[9].eq($v) {
            Some(HeavenlyStems::Tenth)
        } else {
            None
        }
    };
}

impl HeavenlyStems {
    /// 透過甲、乙、丙、丁、戊、己、更、辛、壬、葵等字串來取得 `HeavenlyStems` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<HeavenlyStems> {
        let s = s.as_ref();

        the_heavenly_stems_from!(THE_HEAVENLY_STEMS, s)
    }

    /// 取得 `HeavenlyStems` 列舉實體所代表的地支字串。
    pub fn to_str(&self) -> &'static str {
        let i = *self as usize;

        THE_HEAVENLY_STEMS[i]
    }

    /// 透過甲、乙、丙、丁、戊、己、更、辛、壬、葵等字元來取得 `HeavenlyStems` 列舉實體。
    pub fn from_char(c: char) -> Option<HeavenlyStems> {
        the_heavenly_stems_from!(THE_HEAVENLY_STEMS_CHARS, &c)
    }

    /// 取得 `HeavenlyStems` 列舉實體所代表的地支字元。
    pub fn to_char(&self) -> char {
        let i = *self as usize;

        THE_HEAVENLY_STEMS_CHARS[i]
    }
}

impl Display for HeavenlyStems {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}