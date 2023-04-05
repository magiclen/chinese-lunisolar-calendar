use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "ba-zi-weight")]
use super::BA_ZI_WEIGHT_YEARS;
use super::{EarthlyBranch, HeavenlyStems, Zodiac, THE_LUNAR_YEARS};

/// 農曆年份，由天干加地支組成，六十年一輪。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunarYear {
    heavenly_stems: HeavenlyStems,
    earthly_branch: EarthlyBranch,
    year_index:     usize,
}

impl LunarYear {
    /// 透過農曆年份字串來取得 `LunarYear` 實體。
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarYear> {
        let s = s.as_ref();

        for (i, &t) in THE_LUNAR_YEARS.iter().enumerate() {
            if t == s {
                return Some(unsafe {
                    LunarYear {
                        heavenly_stems: HeavenlyStems::from_ordinal_unsafe(i as i8 % 10),
                        earthly_branch: EarthlyBranch::from_ordinal_unsafe(i as i8 % 12),
                        year_index:     i,
                    }
                });
            }
        }

        None
    }

    /// 取得 `LunarYear` 實體所代表的農曆年份字串。
    #[inline]
    pub fn to_str(self) -> &'static str {
        THE_LUNAR_YEARS[self.year_index]
    }

    /// 透過中國天干地支來取得 `LunarYear` 實體。
    #[inline]
    pub fn from_era(heavenly_stems: HeavenlyStems, earthly_branch: EarthlyBranch) -> LunarYear {
        let h = heavenly_stems as usize;
        let e = earthly_branch as usize;

        let year_index = match h.cmp(&e) {
            Ordering::Equal => h,
            Ordering::Less => {
                let diff = e - h;

                h + (12 - diff) * 5
            },
            Ordering::Greater => {
                let diff = h - e;

                e + diff * 6
            },
        };

        LunarYear {
            heavenly_stems,
            earthly_branch,
            year_index,
        }
    }

    /// 取得天干。
    #[inline]
    pub fn get_heavenly_stems(&self) -> HeavenlyStems {
        self.heavenly_stems
    }

    /// 取得地支。
    #[inline]
    pub fn get_earthly_branch(&self) -> EarthlyBranch {
        self.earthly_branch
    }

    /// 取得生肖。
    #[inline]
    pub fn get_zodiac(&self) -> Zodiac {
        self.earthly_branch.to_zodiac()
    }

    #[cfg(feature = "ba-zi-weight")]
    /// 取得八字重量。
    #[inline]
    pub fn get_ba_zi_weight(&self) -> u8 {
        BA_ZI_WEIGHT_YEARS[self.year_index]
    }
}

impl Display for LunarYear {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}

impl FromStr for LunarYear {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LunarYear::from_str(s).ok_or(())
    }
}
