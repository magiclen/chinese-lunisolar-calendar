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

impl Zodiac {
    pub unsafe fn from_ordinal_unsafe(number: i8) -> Zodiac {
        transmute(number)
    }

    /// 透過鼠、牛、虎、兔、龍、蛇、馬、羊、猴、雞、狗、豬等字串來取得 `Zodiac` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Zodiac> {
        let s = &s.as_ref();

        for (i, &t) in THE_ZODIAC_SIGNS.iter().enumerate() {
            if t.contains(s) {
                return Some(unsafe {
                    Self::from_ordinal_unsafe(i as i8)
                });
            }
        }

        None
    }

    /// 取得 `Zodiac` 列舉實體所代表的生肖字串。
    pub fn to_str(&self, chinese_variant: ChineseVariant) -> &'static str {
        let i = *self as usize;

        match chinese_variant {
            ChineseVariant::Simple => {
                THE_ZODIAC_SIGNS[i][1]
            }
            ChineseVariant::Traditional => {
                THE_ZODIAC_SIGNS[i][0]
            }
        }
    }

    /// 透過鼠、牛、虎、兔、龍、蛇、馬、羊、猴、雞、狗、豬等字元來取得 `Zodiac` 列舉實體。
    pub fn from_char(c: char) -> Option<Zodiac> {
        let c = &c;

        for (i, &t) in THE_ZODIAC_SIGNS_CHARS.iter().enumerate() {
            if t.contains(c) {
                return Some(unsafe {
                    Self::from_ordinal_unsafe(i as i8)
                });
            }
        }

        None
    }

    /// 取得 `Zodiac` 列舉實體所代表的生肖字元。
    pub fn to_char(&self, chinese_variant: ChineseVariant) -> char {
        let i = *self as usize;

        match chinese_variant {
            ChineseVariant::Simple => {
                THE_ZODIAC_SIGNS_CHARS[i][1]
            }
            ChineseVariant::Traditional => {
                THE_ZODIAC_SIGNS_CHARS[i][0]
            }
        }
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