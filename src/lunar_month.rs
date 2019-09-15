use super::{
    ChineseVariant, LunisolarYear, BA_ZI_WEIGHT_MONTHS, BIG_MONTHS, MIN_YEAR_IN_SOLAR_CALENDAR,
    THE_LUNAR_MONTHS,
};

use std::mem::transmute;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

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

impl LunarMonth {
    pub unsafe fn from_ordinal_unsafe(number: i8) -> LunarMonth {
        transmute(number)
    }

    /// 透過農曆月份字串來取得 `LunarMonth` 列舉實體。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarMonth> {
        let s = &s.as_ref();

        for (i, &t) in THE_LUNAR_MONTHS.iter().enumerate().take(24) {
            if t.contains(s) {
                if i >= 12 {
                    return Some(unsafe { Self::from_ordinal_unsafe((i as i8 - 12) * 2 + 1) });
                } else {
                    return Some(unsafe { Self::from_ordinal_unsafe(i as i8 * 2) });
                }
            }
        }

        if THE_LUNAR_MONTHS[24].contains(s) {
            Some(LunarMonth::First)
        } else if THE_LUNAR_MONTHS[25].contains(s) {
            Some(LunarMonth::Eleventh)
        } else if THE_LUNAR_MONTHS[26].contains(s) {
            Some(LunarMonth::Twelfth)
        } else if THE_LUNAR_MONTHS[27].contains(s) {
            Some(LunarMonth::LeapFirst)
        } else if THE_LUNAR_MONTHS[28].contains(s) {
            Some(LunarMonth::LeapEleventh)
        } else if THE_LUNAR_MONTHS[29].contains(s) || THE_LUNAR_MONTHS[30].contains(s) {
            Some(LunarMonth::LeapTwelfth)
        } else {
            None
        }
    }

    /// 取得 `LunarMonth` 列舉實體所代表的農曆月份字串。
    pub fn to_str(self, chinese_variant: ChineseVariant) -> &'static str {
        let mut i = self as usize;

        if i % 2 == 1 {
            i = i / 2 + 12;
        } else {
            i /= 2;
        }

        match chinese_variant {
            ChineseVariant::Simple => THE_LUNAR_MONTHS[i][1],
            ChineseVariant::Traditional => THE_LUNAR_MONTHS[i][0],
        }
    }

    /// 透過農曆月份數值和是否閏月來取得 `LunarMonth` 列舉實體。
    pub unsafe fn from_u8_unsafe(month: u8, leap: bool) -> LunarMonth {
        if leap {
            transmute((month - 1) * 2 + 1)
        } else {
            transmute((month - 1) * 2)
        }
    }

    /// 透過農曆月份數值和是否閏月來取得 `LunarMonth` 列舉實體。
    pub fn from_u8(month: u8, leap: bool) -> Option<LunarMonth> {
        if month == 0 || month > 12 {
            None
        } else {
            Some(unsafe { Self::from_u8_unsafe(month, leap) })
        }
    }

    /// 取得 `LunarMonth` 列舉實體所代表的農曆月份數值。
    pub fn to_u8(self) -> u8 {
        let i = self as u8;

        i / 2 + 1
    }

    /// 是否為閏月。
    pub fn is_leap_month(self) -> bool {
        let i = self as usize;

        i % 2 == 1
    }

    /// 傳入指定的農曆西曆年，並計算此農曆月在這個指定的農曆西曆年內共有幾天。
    pub fn get_total_days(self, lunisolar_year: LunisolarYear) -> Option<u8> {
        let mut month = self.to_u8();

        let year = lunisolar_year.to_u16();

        let leap_month = match lunisolar_year.get_leap_lunar_month() {
            Some(leap_lunar_month) => leap_lunar_month.to_u8(),
            None => 0,
        };

        if self.is_leap_month() {
            if month != leap_month {
                // 防呆
                None
            } else {
                // 此為閏月，需計算其後一個月的天數
                if (BIG_MONTHS[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize]
                    & (0x8000 >> u16::from(leap_month)))
                    == 0
                {
                    Some(29)
                } else {
                    Some(30)
                }
            }
        } else {
            if leap_month > 0 && month > leap_month {
                // 若今年有閏月，且該西曆月應在閏月之後再加一個月
                month += 1;
            }
            if (BIG_MONTHS[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize]
                & (0x8000 >> u16::from(month - 1)))
                == 0
            {
                Some(29)
            } else {
                Some(30)
            }
        }
    }

    /// 取得八字重量。
    pub fn get_ba_zi_weight(self) -> u8 {
        let i = self.to_u8() as usize;

        BA_ZI_WEIGHT_MONTHS[i - 1]
    }
}

impl Display for LunarMonth {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str(ChineseVariant::Traditional))
    }
}

impl FromStr for LunarMonth {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LunarMonth::from_str(s).ok_or(())
    }
}
