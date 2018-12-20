use super::{SolarYear, HeavenlyStems, EarthlyBranch, Zodiac, LunarYear, MAX_LUNAR_DATE_IN_SOLAR_CALENDAR, MIN_LUNAR_DATE_IN_SOLAR_CALENDAR};

use std::fmt::{self, Display, Formatter};

use chrono::prelude::*;

/// 農曆新年所在的西曆年份。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunisolarYear {
    solar_year: SolarYear,
}

impl LunisolarYear {
    /// 透過西曆年份來取得 `LunisolarYear` 實體。
    pub unsafe fn from_solar_year_unsafe<Y: Into<SolarYear>>(solar_year: Y) -> LunisolarYear {
        let solar_year = solar_year.into();

        LunisolarYear {
            solar_year
        }
    }

    /// 透過西曆年份來取得 `LunisolarYear` 實體。
    pub fn from_solar_year<Y: Into<SolarYear>>(solar_year: Y) -> Option<LunisolarYear> {
        let solar_year = solar_year.into();

        let year = solar_year.to_u16();

        let min_year = MIN_LUNAR_DATE_IN_SOLAR_CALENDAR.year() as u16;
        let max_year = MAX_LUNAR_DATE_IN_SOLAR_CALENDAR.year() as u16;

        if year >= min_year && max_year <= max_year {
            Some(LunisolarYear {
                solar_year
            })
        } else {
            None
        }
    }

    /// 取得此西曆年中，農曆新年的中國天干。
    pub fn get_heavenly_stems(&self) -> HeavenlyStems {
        let index = (6 + (self.solar_year.to_u16() - 1900)) % 10;

        unsafe {
            HeavenlyStems::from_ordinal_unsafe(index as i8)
        }
    }

    /// 取得此西曆年中，農曆新年的中國地支。
    pub fn get_earthly_branch(&self) -> EarthlyBranch {
        let index = (self.solar_year.to_u16() - 1900) % 12;

        unsafe {
            EarthlyBranch::from_ordinal_unsafe(index as i8)
        }
    }

    /// 取得此西曆年中，農曆新年所屬的生肖。
    pub fn get_zodiac(&self) -> Zodiac {
        let index = (self.solar_year.to_u16() - 1900) % 12;

        unsafe {
            Zodiac::from_ordinal_unsafe(index as i8)
        }
    }

    /// 取得 `LunarYear` 實體。
    pub fn to_lunar_year(&self) -> LunarYear {
        let heavenly_stems = self.get_heavenly_stems();
        let earthly_branch = self.get_earthly_branch();

        LunarYear::from_era(heavenly_stems, earthly_branch)
    }

    /// 取得 `SolarYear` 實體。
    pub fn to_solar_year(&self) -> SolarYear {
        self.solar_year
    }

    /// 取得 `LunisolarYear` 實體所代表的西曆年份數值。
    pub fn to_u16(&self) -> u16 {
        self.solar_year.to_u16()
    }
}

impl Display for LunisolarYear {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(&self.solar_year, f)
    }
}