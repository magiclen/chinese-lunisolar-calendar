use super::{Zodiac, THE_EARTHLY_BRANCHES, THE_EARTHLY_BRANCHES_CHARS};

#[cfg(feature = "ba-zi-weight")]
use super::BA_ZI_WEIGHT_TIME;

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use chrono::prelude::*;

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
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_ordinal_unsafe(number: i8) -> EarthlyBranch {
        transmute(number)
    }

    /// 將時間轉成對應的地支。
    #[inline]
    pub fn from_time<T: Timelike>(time: T) -> EarthlyBranch {
        let hour = time.hour();

        let ordinal = ((hour + 1) % 24) / 2;

        unsafe { Self::from_ordinal_unsafe(ordinal as i8) }
    }

    /// 透過子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥等字串來取得 `EarthlyBranch` 列舉實體。
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<EarthlyBranch> {
        let s = s.as_ref();

        for (i, &t) in THE_EARTHLY_BRANCHES.iter().enumerate() {
            if t == s {
                return Some(unsafe { Self::from_ordinal_unsafe(i as i8) });
            }
        }

        None
    }

    /// 取得 `EarthlyBranch` 列舉實體所代表的地支字串。
    #[inline]
    pub fn to_str(self) -> &'static str {
        let i = self as usize;

        THE_EARTHLY_BRANCHES[i]
    }

    /// 透過子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥等字元來取得 `EarthlyBranch` 列舉實體。
    #[inline]
    pub fn from_char(c: char) -> Option<EarthlyBranch> {
        for (i, t) in THE_EARTHLY_BRANCHES_CHARS.iter().copied().enumerate() {
            if t == c {
                return Some(unsafe { Self::from_ordinal_unsafe(i as i8) });
            }
        }

        None
    }

    /// 取得 `EarthlyBranch` 列舉實體所代表的地支字元。
    #[inline]
    pub fn to_char(self) -> char {
        let i = self as usize;

        THE_EARTHLY_BRANCHES_CHARS[i]
    }

    /// 透過生肖來取得地支。
    #[inline]
    pub fn from_zodiac(zodiac: Zodiac) -> EarthlyBranch {
        unsafe { transmute(zodiac) }
    }

    /// 將地支轉成生肖。
    #[inline]
    pub fn to_zodiac(self) -> Zodiac {
        unsafe { transmute(self) }
    }

    #[cfg(feature = "ba-zi-weight")]
    /// 取得八字重量。
    #[inline]
    pub fn get_ba_zi_weight(self) -> u8 {
        let i = self as usize;

        BA_ZI_WEIGHT_TIME[i]
    }
}

impl Display for EarthlyBranch {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}

impl From<Zodiac> for EarthlyBranch {
    #[inline]
    fn from(zodiac: Zodiac) -> EarthlyBranch {
        EarthlyBranch::from_zodiac(zodiac)
    }
}

impl FromStr for EarthlyBranch {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EarthlyBranch::from_str(s).ok_or(())
    }
}
