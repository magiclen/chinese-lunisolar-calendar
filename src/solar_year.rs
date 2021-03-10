use super::{
    SolarMonth, THE_SOLAR_YEAR_NUMBERS, THE_SOLAR_YEAR_NUMBERS_2, THE_SOLAR_YEAR_NUMBERS_CHARS,
};

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// 西曆年份。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct SolarYear {
    year: u16,
}

impl SolarYear {
    /// 透過西曆年份字串來取得 `SolarYear` 實體。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<SolarYear> {
        let s = s.as_ref();

        let mut year = 0;

        for (count, c) in s.chars().enumerate() {
            if count == 5 {
                return None;
            }

            if ('0'..='9').contains(&c) {
                year = year * 10 + u32::from(c as u8 - b'0');
            } else {
                let mut failed = true;
                for (i, cc) in THE_SOLAR_YEAR_NUMBERS_CHARS.iter().copied().enumerate() {
                    if c == cc {
                        year = year * 10 + i as u32;
                        failed = false;
                        break;
                    }
                }

                if c == '〇' {
                    year *= 10;
                } else if failed {
                    return None;
                }
            }
        }

        if year > u32::from(u16::max_value()) {
            return None;
        }

        Some(SolarYear {
            year: year as u16,
        })
    }

    /// 取得 `SolarYear` 實體所代表的中文西曆年份字串。以`零`表示數字`0`。
    #[inline]
    pub fn to_chinese_string(self) -> String {
        let mut year_string = String::new();

        self.write_to_chinese_string(&mut year_string);

        year_string
    }

    /// 取得 `SolarYear` 實體所代表的中文西曆年份字串。以`〇`表示數字`0`。
    #[inline]
    pub fn to_chinese_string_2(self) -> String {
        let mut year_string = String::new();

        self.write_to_chinese_string_2(&mut year_string);

        year_string
    }

    /// 取得 `SolarYear` 實體所代表的中文西曆年份字串。以`零`表示數字`0`。
    #[inline]
    pub fn write_to_chinese_string(self, s: &mut String) {
        let mut year = self.year;

        if year == 0 {
            s.push_str(THE_SOLAR_YEAR_NUMBERS[0]);
        } else {
            s.reserve(12);

            let len = s.len();

            loop {
                let digit = year % 10;
                year /= 10;

                s.insert_str(len, THE_SOLAR_YEAR_NUMBERS[digit as usize]);

                if year == 0 {
                    break;
                }
            }
        }
    }

    /// 取得 `SolarYear` 實體所代表的中文西曆年份字串。以`〇`表示數字`0`。
    #[inline]
    pub fn write_to_chinese_string_2(self, s: &mut String) {
        let mut year = self.year;

        if year == 0 {
            s.push_str(THE_SOLAR_YEAR_NUMBERS_2[0]);
        } else {
            s.reserve(12);

            let len = s.len();

            loop {
                let digit = year % 10;
                year /= 10;

                s.insert_str(len, THE_SOLAR_YEAR_NUMBERS_2[digit as usize]);

                if year == 0 {
                    break;
                }
            }
        }
    }

    /// 透過西曆年份數值來取得 `SolarYear` 實體。
    #[inline]
    pub fn from_u16(year: u16) -> SolarYear {
        SolarYear {
            year,
        }
    }

    /// 取得 `SolarYear` 實體所代表的西曆年份數值。
    #[inline]
    pub fn to_u16(self) -> u16 {
        self.year
    }

    /// 判斷此西曆年是否為閏年。
    #[inline]
    pub fn is_leap(self) -> bool {
        (self.year % 4 == 0) && (self.year % 100 != 0) || self.year % 400 == 0
    }

    /// 計算此西曆年共有幾天。。
    #[inline]
    pub fn get_total_days(self) -> u16 {
        if self.is_leap() {
            366
        } else {
            365
        }
    }

    /// 計算此西曆年下的某個月共有幾天。。
    #[inline]
    pub fn get_total_days_in_a_month(self, solar_month: SolarMonth) -> u8 {
        solar_month.get_total_days(self)
    }
}

impl Display for SolarYear {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_chinese_string())
    }
}

impl From<u16> for SolarYear {
    #[inline]
    fn from(year: u16) -> SolarYear {
        SolarYear::from_u16(year)
    }
}

impl FromStr for SolarYear {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SolarYear::from_str(s).ok_or(())
    }
}
