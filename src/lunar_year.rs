use super::{THE_LUNAR_YEARS, HeavenlyStems, EarthlyBranch, Zodiac, BA_ZI_WEIGHT_YEARS};

use std::fmt::{self, Display, Formatter};

/// 農曆年份，由天干加地支組成，六十年一輪。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunarYear {
    heavenly_stems: HeavenlyStems,
    earthly_branch: EarthlyBranch,
    year_index: usize,
}

impl LunarYear {
    /// 透過農曆年份字串來取得 `LunarYear` 實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarYear> {
        let s = s.as_ref();

        for (i, &t) in THE_LUNAR_YEARS.iter().enumerate() {
            if t.eq(s) {
                return Some(unsafe {
                    LunarYear {
                        heavenly_stems: HeavenlyStems::from_ordinal_unsafe(i as i8 % 10),
                        earthly_branch: EarthlyBranch::from_ordinal_unsafe(i as i8 % 12),
                        year_index: i,
                    }
                });
            }
        }

        None
    }

    /// 取得 `LunarYear` 實體所代表的農曆年份字串。
    pub fn to_str(&self) -> &'static str {
        THE_LUNAR_YEARS[self.year_index]
    }

    /// 透過中國天干地支來取得 `LunarYear` 實體。
    pub fn from_era(heavenly_stems: HeavenlyStems, earthly_branch: EarthlyBranch) -> LunarYear {
        let h = heavenly_stems as usize;
        let e = earthly_branch as usize;
        let mut diff = e - h;

        if diff <= 0 {
            diff += 12;
        }

        let year_index = h + (12 - diff) * 5;

        LunarYear {
            heavenly_stems,
            earthly_branch,
            year_index,
        }
    }

    /// 取得天干。
    pub fn get_heavenly_stems(&self) -> HeavenlyStems {
        self.heavenly_stems
    }

    /// 取得地支。
    pub fn get_earthly_branch(&self) -> EarthlyBranch {
        self.earthly_branch
    }

    /// 取得生肖。
    pub fn get_zodiac(&self) -> Zodiac {
        self.earthly_branch.to_zodiac()
    }

    /// 取得八字重量。
    pub fn get_ba_zi_weight(&self) -> u8 {
        BA_ZI_WEIGHT_YEARS[self.year_index]
    }
}

impl Display for LunarYear {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}