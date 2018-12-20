use super::{THE_SOLAR_YEAR_NUMBERS, THE_SOLAR_YEAR_NUMBERS_CHARS};

use std::fmt::{self, Display, Formatter};

/// 西曆年份。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct SolarYear {
    /// 西曆年
    pub(crate) year: u16
}

impl SolarYear {
    /// 透過西曆年份字串來取得 `SolarYear` 實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<SolarYear> {
        let s = s.as_ref();

        let mut year = 0;

        let mut count = 0;

        for c in s.chars() {
            if count == 5 {
                return None;
            }

            if c >= '0' && c <= '9' {
                year = year * 10 + (c as u8 - b'0') as u32;
            } else {
                let mut failed = true;
                for (i, &cc) in THE_SOLAR_YEAR_NUMBERS_CHARS.iter().enumerate() {
                    if c == cc {
                        year = year * 10 + i as u32;
                        failed = false;
                        break;
                    }
                }

                if failed {
                    return None;
                }
            }

            count += 1;
        }

        if year > u16::max_value() as u32 {
            return None;
        }

        Some(SolarYear {
            year: year as u16
        })
    }

    /// 取得 `SolarYear` 實體所代表的中文年份字串。
    pub fn to_chinese_string(&self) -> String {
        let mut year_string = String::new();

        self.write_to_chinese_string(&mut year_string);

        year_string
    }

    /// 取得 `SolarYear` 實體所代表的中文年份字串。
    pub fn write_to_chinese_string(&self, s: &mut String) {
        let mut year = self.year;

        s.reserve(12);

        let len = s.len();

        while year > 0 {
            let digit = year % 10;
            year /= 10;

            s.insert_str(len, THE_SOLAR_YEAR_NUMBERS[digit as usize]);
        }
    }

    /// 透過農曆日期數值來取得 `SolarYear` 實體。
    pub fn from_u16(year: u16) -> SolarYear {
        SolarYear {
            year
        }
    }

    /// 取得 `SolarYear` 實體所代表的西曆年份數值。
    pub fn to_u16(&self) -> u16 {
        self.year
    }
}

impl Display for SolarYear {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_chinese_string())
    }
}

impl From<u16> for SolarYear {
    fn from(year: u16) -> SolarYear {
        SolarYear::from(year)
    }
}