use super::{
    EarthlyBranch, HeavenlyStems, LunarMonth, LunarYear, SolarYear, Zodiac, BIG_MONTHS,
    LEAP_MONTHS, MAX_YEAR_IN_SOLAR_CALENDAR, MIN_YEAR_IN_SOLAR_CALENDAR,
};

use std::fmt::{self, Display, Formatter};

/// 農曆西曆年，農曆新年所在的西曆年份。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunisolarYear {
    solar_year: SolarYear,
}

impl LunisolarYear {
    /// 透過西曆年份來取得 `LunisolarYear` 實體。
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn from_solar_year_unsafe<Y: Into<SolarYear>>(solar_year: Y) -> LunisolarYear {
        let solar_year = solar_year.into();

        LunisolarYear {
            solar_year,
        }
    }

    /// 透過西曆年份來取得 `LunisolarYear` 實體。
    #[inline]
    pub fn from_solar_year<Y: Into<SolarYear>>(solar_year: Y) -> Option<LunisolarYear> {
        let solar_year = solar_year.into();

        let year = solar_year.to_u16();

        if year >= MIN_YEAR_IN_SOLAR_CALENDAR && year <= MAX_YEAR_IN_SOLAR_CALENDAR {
            Some(LunisolarYear {
                solar_year,
            })
        } else {
            None
        }
    }

    /// 取得此西曆年中，農曆新年的中國天干。
    #[inline]
    pub fn get_heavenly_stems(self) -> HeavenlyStems {
        let index = (7 + (self.solar_year.to_u16() - MIN_YEAR_IN_SOLAR_CALENDAR)) % 10;

        unsafe { HeavenlyStems::from_ordinal_unsafe(index as i8) }
    }

    /// 取得此西曆年中，農曆新年的中國地支。
    #[inline]
    pub fn get_earthly_branch(self) -> EarthlyBranch {
        let index = (self.solar_year.to_u16() - MIN_YEAR_IN_SOLAR_CALENDAR + 1) % 12;

        unsafe { EarthlyBranch::from_ordinal_unsafe(index as i8) }
    }

    /// 取得此西曆年中，農曆新年所屬的生肖。
    #[inline]
    pub fn get_zodiac(self) -> Zodiac {
        let index = (self.solar_year.to_u16() - MIN_YEAR_IN_SOLAR_CALENDAR + 1) % 12;

        unsafe { Zodiac::from_ordinal_unsafe(index as i8) }
    }

    /// 取得此年的農曆閏月月份。
    #[inline]
    pub fn get_leap_lunar_month(self) -> Option<LunarMonth> {
        let year = self.to_u16();

        let month = LEAP_MONTHS[((year - MIN_YEAR_IN_SOLAR_CALENDAR) / 2) as usize];

        let index = if year % 2 == 1 {
            (month & 0xf0) >> 4
        } else {
            month & 0x0f
        };

        if index == 0 {
            None
        } else {
            Some(unsafe { LunarMonth::from_u8_unsafe(index, true) })
        }
    }

    /// 計算此西曆年下的農曆閏月共有幾天。如果沒有閏月，則回傳0。
    #[inline]
    pub fn get_total_days_in_leap_month(self) -> u16 {
        let leap_lunar_month = self.get_leap_lunar_month();

        match leap_lunar_month {
            Some(leap_lunar_month) => self.get_total_days_in_leap_month_inner(leap_lunar_month),
            None => 0,
        }
    }

    /// 計算指定的農曆閏月共有幾天。
    #[inline]
    pub(crate) fn get_total_days_in_leap_month_inner(self, leap_lunar_month: LunarMonth) -> u16 {
        let year = self.to_u16();

        let leap_month = leap_lunar_month.to_u8();

        if (BIG_MONTHS[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize]
            & (0x8000 >> u16::from(leap_month)))
            == 0
        {
            29
        } else {
            30
        }
    }

    /// 計算此西曆年下的農曆年共有幾天。。
    pub fn get_total_days(self) -> u16 {
        let leap_lunar_month = self.get_leap_lunar_month();

        let (leap_month, mut n) = match leap_lunar_month {
            Some(leap_lunar_month) => {
                (
                    leap_lunar_month.to_u8(),
                    self.get_total_days_in_leap_month_inner(leap_lunar_month),
                )
            }
            None => (0, 0),
        };

        let year = self.to_u16();

        if leap_month > 0 {
            for month in 1..=leap_month {
                if (BIG_MONTHS[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize]
                    & (0x8000 >> u16::from(month - 1)))
                    == 0
                {
                    n += 29;
                } else {
                    n += 30;
                }
            }

            for month in (leap_month + 1)..=12 {
                if (BIG_MONTHS[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize]
                    & (0x8000 >> u16::from(month)))
                    == 0
                {
                    n += 29;
                } else {
                    n += 30;
                }
            }
        } else {
            for month in 1..=12 {
                if (BIG_MONTHS[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize]
                    & (0x8000 >> (month - 1)))
                    == 0
                {
                    n += 29;
                } else {
                    n += 30;
                }
            }
        }

        n
    }

    /// 計算此西曆年下的農曆年的某個月共有幾天。。
    #[inline]
    pub fn get_total_days_in_a_month(self, lunar_month: LunarMonth) -> Option<u8> {
        lunar_month.get_total_days(self)
    }

    /// 取得 `LunarYear` 實體。
    #[inline]
    pub fn to_lunar_year(self) -> LunarYear {
        let heavenly_stems = self.get_heavenly_stems();
        let earthly_branch = self.get_earthly_branch();

        LunarYear::from_era(heavenly_stems, earthly_branch)
    }

    /// 取得 `SolarYear` 實體。
    #[inline]
    pub fn to_solar_year(self) -> SolarYear {
        self.solar_year
    }

    /// 取得 `LunisolarYear` 實體所代表的西曆年份數值。
    #[inline]
    pub fn to_u16(self) -> u16 {
        self.solar_year.to_u16()
    }
}

impl Display for LunisolarYear {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.solar_year, f)
    }
}
