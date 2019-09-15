use super::{THE_HEAVENLY_STEMS, THE_HEAVENLY_STEMS_CHARS};

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

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

impl HeavenlyStems {
    pub unsafe fn from_ordinal_unsafe(number: i8) -> HeavenlyStems {
        transmute(number)
    }

    /// 透過甲、乙、丙、丁、戊、己、更、辛、壬、葵等字串來取得 `HeavenlyStems` 列舉實體。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<HeavenlyStems> {
        let s = s.as_ref();

        for (i, &t) in THE_HEAVENLY_STEMS.iter().enumerate() {
            if t.eq(s) {
                return Some(unsafe { Self::from_ordinal_unsafe(i as i8) });
            }
        }

        None
    }

    /// 取得 `HeavenlyStems` 列舉實體所代表的地支字串。
    pub fn to_str(self) -> &'static str {
        let i = self as usize;

        THE_HEAVENLY_STEMS[i]
    }

    /// 透過甲、乙、丙、丁、戊、己、更、辛、壬、葵等字元來取得 `HeavenlyStems` 列舉實體。
    pub fn from_char(c: char) -> Option<HeavenlyStems> {
        for (i, &t) in THE_HEAVENLY_STEMS_CHARS.iter().enumerate() {
            if t.eq(&c) {
                return Some(unsafe { Self::from_ordinal_unsafe(i as i8) });
            }
        }

        None
    }

    /// 取得 `HeavenlyStems` 列舉實體所代表的地支字元。
    pub fn to_char(self) -> char {
        let i = self as usize;

        THE_HEAVENLY_STEMS_CHARS[i]
    }
}

impl Display for HeavenlyStems {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}

impl FromStr for HeavenlyStems {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HeavenlyStems::from_str(s).ok_or(())
    }
}
