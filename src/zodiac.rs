use super::{ChineseVariant, THE_ZODIAC_SIGNS, THE_ZODIAC_SIGNS_CHARS, EarthlyBranch};

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};

/// 列舉中國十二生肖：鼠、牛、虎、兔、龍、蛇、馬、羊、猴、雞、狗、豬。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub enum Zodiac {
    /// 鼠
    Rat,
    /// 牛
    Ox,
    /// 虎
    Tiger,
    /// 兔
    Rabbit,
    /// 龍
    Dragon,
    /// 蛇
    Snake,
    /// 馬
    Horse,
    /// 羊
    Goat,
    /// 猴
    Monkey,
    /// 雞
    Rooster,
    /// 狗
    Dog,
    /// 豬
    Pig,
}

macro_rules! the_zodiac_signs_contains {
    ($a:expr, $v:expr) => {
        if $a[0].contains($v) {
            Some(Zodiac::Rat)
        } else if $a[1].contains($v) {
            Some(Zodiac::Ox)
        } else if $a[2].contains($v) {
            Some(Zodiac::Tiger)
        } else if $a[3].contains($v) {
            Some(Zodiac::Rabbit)
        } else if $a[4].contains($v) {
            Some(Zodiac::Dragon)
        } else if $a[5].contains($v) {
            Some(Zodiac::Snake)
        } else if $a[6].contains($v) {
            Some(Zodiac::Horse)
        } else if $a[7].contains($v) {
            Some(Zodiac::Goat)
        } else if $a[8].contains($v) {
            Some(Zodiac::Monkey)
        } else if $a[9].contains($v) {
            Some(Zodiac::Rooster)
        } else if $a[10].contains($v) {
            Some(Zodiac::Dog)
        } else if $a[11].contains($v) {
            Some(Zodiac::Pig)
        } else {
            None
        }
    };
}

macro_rules! the_zodiac_signs_variants {
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

impl Zodiac {
    /// 透過鼠、牛、虎、兔、龍、蛇、馬、羊、猴、雞、狗、豬等字串來取得 `Zodiac` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Zodiac> {
        let s = &s.as_ref();

        the_zodiac_signs_contains!(THE_ZODIAC_SIGNS, s)
    }

    /// 取得 `Zodiac` 列舉實體所代表的生肖字串。
    pub fn to_str(&self, chinese_variant: ChineseVariant) -> &'static str {
        let i = *self as usize;

        the_zodiac_signs_variants!(THE_ZODIAC_SIGNS, chinese_variant, i)
    }

    /// 透過鼠、牛、虎、兔、龍、蛇、馬、羊、猴、雞、狗、豬等字元來取得 `Zodiac` 列舉實體。
    pub fn from_char(c: char) -> Option<Zodiac> {
        the_zodiac_signs_contains!(THE_ZODIAC_SIGNS_CHARS, &c)
    }

    /// 取得 `Zodiac` 列舉實體所代表的生肖字元。
    pub fn to_char(&self, chinese_variant: ChineseVariant) -> char {
        let i = *self as usize;

        the_zodiac_signs_variants!(THE_ZODIAC_SIGNS_CHARS, chinese_variant, i)
    }

    /// 透過地支來取得生肖。
    pub fn from_earthly_branch(earthly_branch: EarthlyBranch) -> Zodiac {
        unsafe { transmute(earthly_branch) }
    }

    /// 將生肖轉成地支。
    pub fn to_earthly_branch(&self) -> EarthlyBranch {
        unsafe { transmute(*self) }
    }
}

impl Display for Zodiac {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str(ChineseVariant::Traditional))
    }
}