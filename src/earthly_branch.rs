use super::{THE_EARTHLY_BRANCHES, THE_EARTHLY_BRANCHES_CHARS, Zodiac};

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};

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

impl EarthlyBranch {
    pub unsafe fn from_ordinal_unsafe(number: i8) -> EarthlyBranch {
        transmute(number)
    }

    /// 透過子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥等字串來取得 `EarthlyBranch` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<EarthlyBranch> {
        let s = s.as_ref();

        for (i, &t) in THE_EARTHLY_BRANCHES.iter().enumerate() {
            if t.eq(s) {
                return Some(unsafe {
                    Self::from_ordinal_unsafe(i as i8)
                });
            }
        }

        None
    }

    /// 取得 `EarthlyBranch` 列舉實體所代表的地支字串。
    pub fn to_str(&self) -> &'static str {
        let i = *self as usize;

        THE_EARTHLY_BRANCHES[i]
    }

    /// 透過子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥等字元來取得 `EarthlyBranch` 列舉實體。
    pub fn from_char(c: char) -> Option<EarthlyBranch> {
        for (i, &t) in THE_EARTHLY_BRANCHES_CHARS.iter().enumerate() {
            if t.eq(&c) {
                return Some(unsafe {
                    Self::from_ordinal_unsafe(i as i8)
                });
            }
        }

        None
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

impl Display for EarthlyBranch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}