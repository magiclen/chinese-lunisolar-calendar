use super::{THE_LUNAR_YEARS, HeavenlyStems, EarthlyBranch, Zodiac};

use std::fmt::{self, Display, Formatter};

/// 農曆年份，由天干加地支組成，六十年一輪。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunarYear {
    /// 天干
    pub(crate) heavenly_stems: HeavenlyStems,
    /// 地支
    pub(crate) earthly_branch: EarthlyBranch,
    /// 年份索引
    pub(crate) year_index: usize,
}

macro_rules! the_lunar_years_from {
    ($a:expr, $v:expr) => {
        if $a[0].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::First,
                earthly_branch: EarthlyBranch::First,
                year_index: 0
            })
        } else if $a[1].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Second,
                earthly_branch: EarthlyBranch::Second,
                year_index: 1
            })
        } else if $a[2].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Third,
                earthly_branch: EarthlyBranch::Third,
                year_index: 2
            })
        } else if $a[3].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fourth,
                earthly_branch: EarthlyBranch::Fourth,
                year_index: 3
            })
        } else if $a[4].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fifth,
                earthly_branch: EarthlyBranch::Fifth,
                year_index: 4
            })
        } else if $a[5].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Sixth,
                earthly_branch: EarthlyBranch::Sixth,
                year_index: 5
            })
        } else if $a[6].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Seventh,
                earthly_branch: EarthlyBranch::Seventh,
                year_index: 6
            })
        } else if $a[7].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Eighth,
                earthly_branch: EarthlyBranch::Eighth,
                year_index: 7
            })
        } else if $a[8].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Ninth,
                earthly_branch: EarthlyBranch::Ninth,
                year_index: 8
            })
        } else if $a[9].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Tenth,
                earthly_branch: EarthlyBranch::Tenth,
                year_index: 9
            })
        } else if $a[10].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::First,
                earthly_branch: EarthlyBranch::Eleventh,
                year_index: 10
            })
        } else if $a[11].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Second,
                earthly_branch: EarthlyBranch::Twelfth,
                year_index: 11
            })
        } else if $a[12].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Third,
                earthly_branch: EarthlyBranch::First,
                year_index: 12
            })
        } else if $a[13].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fourth,
                earthly_branch: EarthlyBranch::Second,
                year_index: 13
            })
        } else if $a[14].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fifth,
                earthly_branch: EarthlyBranch::Third,
                year_index: 14
            })
        } else if $a[15].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Sixth,
                earthly_branch: EarthlyBranch::Fourth,
                year_index: 15
            })
        } else if $a[16].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Seventh,
                earthly_branch: EarthlyBranch::Fifth,
                year_index: 16
            })
        } else if $a[17].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Eighth,
                earthly_branch: EarthlyBranch::Sixth,
                year_index: 17
            })
        } else if $a[18].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Ninth,
                earthly_branch: EarthlyBranch::Seventh,
                year_index: 18
            })
        } else if $a[19].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Tenth,
                earthly_branch: EarthlyBranch::Eighth,
                year_index: 19
            })
        } else if $a[20].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::First,
                earthly_branch: EarthlyBranch::Ninth,
                year_index: 20
            })
        } else if $a[21].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Second,
                earthly_branch: EarthlyBranch::Tenth,
                year_index: 21
            })
        } else if $a[22].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Third,
                earthly_branch: EarthlyBranch::Eleventh,
                year_index: 22
            })
        } else if $a[23].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fourth,
                earthly_branch: EarthlyBranch::Twelfth,
                year_index: 23
            })
        } else if $a[24].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fifth,
                earthly_branch: EarthlyBranch::First,
                year_index: 24
            })
        } else if $a[25].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Sixth,
                earthly_branch: EarthlyBranch::Second,
                year_index: 25
            })
        } else if $a[26].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Seventh,
                earthly_branch: EarthlyBranch::Third,
                year_index: 26
            })
        } else if $a[27].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Eighth,
                earthly_branch: EarthlyBranch::Fourth,
                year_index: 27
            })
        } else if $a[28].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Ninth,
                earthly_branch: EarthlyBranch::Fifth,
                year_index: 28
            })
        } else if $a[29].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Tenth,
                earthly_branch: EarthlyBranch::Sixth,
                year_index: 29
            })
        } else if $a[30].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::First,
                earthly_branch: EarthlyBranch::Seventh,
                year_index: 30
            })
        } else if $a[31].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Second,
                earthly_branch: EarthlyBranch::Eighth,
                year_index: 31
            })
        } else if $a[32].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Third,
                earthly_branch: EarthlyBranch::Ninth,
                year_index: 32
            })
        } else if $a[33].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fourth,
                earthly_branch: EarthlyBranch::Tenth,
                year_index: 33
            })
        } else if $a[34].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fifth,
                earthly_branch: EarthlyBranch::Eleventh,
                year_index: 34
            })
        } else if $a[35].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Sixth,
                earthly_branch: EarthlyBranch::Twelfth,
                year_index: 35
            })
        } else if $a[36].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Seventh,
                earthly_branch: EarthlyBranch::First,
                year_index: 36
            })
        } else if $a[37].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Eighth,
                earthly_branch: EarthlyBranch::Second,
                year_index: 37
            })
        } else if $a[38].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Ninth,
                earthly_branch: EarthlyBranch::Third,
                year_index: 38
            })
        } else if $a[39].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Tenth,
                earthly_branch: EarthlyBranch::Fourth,
                year_index: 39
            })
        } else if $a[40].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::First,
                earthly_branch: EarthlyBranch::Fifth,
                year_index: 40
            })
        } else if $a[41].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Second,
                earthly_branch: EarthlyBranch::Sixth,
                year_index: 41
            })
        } else if $a[42].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Third,
                earthly_branch: EarthlyBranch::Seventh,
                year_index: 42
            })
        } else if $a[43].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fourth,
                earthly_branch: EarthlyBranch::Eighth,
                year_index: 43
            })
        } else if $a[44].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fifth,
                earthly_branch: EarthlyBranch::Ninth,
                year_index: 44
            })
        } else if $a[45].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Sixth,
                earthly_branch: EarthlyBranch::Tenth,
                year_index: 45
            })
        } else if $a[46].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Seventh,
                earthly_branch: EarthlyBranch::Eleventh,
                year_index: 46
            })
        } else if $a[47].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Eighth,
                earthly_branch: EarthlyBranch::Twelfth,
                year_index: 47
            })
        } else if $a[48].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Ninth,
                earthly_branch: EarthlyBranch::First,
                year_index: 48
            })
        } else if $a[49].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Tenth,
                earthly_branch: EarthlyBranch::Second,
                year_index: 49
            })
        } else if $a[50].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::First,
                earthly_branch: EarthlyBranch::Third,
                year_index: 50
            })
        } else if $a[51].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Second,
                earthly_branch: EarthlyBranch::Fourth,
                year_index: 51
            })
        } else if $a[52].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Third,
                earthly_branch: EarthlyBranch::Fifth,
                year_index: 52
            })
        } else if $a[53].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fourth,
                earthly_branch: EarthlyBranch::Sixth,
                year_index: 53
            })
        } else if $a[54].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Fifth,
                earthly_branch: EarthlyBranch::Seventh,
                year_index: 54
            })
        } else if $a[55].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Sixth,
                earthly_branch: EarthlyBranch::Eighth,
                year_index: 55
            })
        } else if $a[56].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Seventh,
                earthly_branch: EarthlyBranch::Ninth,
                year_index: 56
            })
        } else if $a[57].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Eighth,
                earthly_branch: EarthlyBranch::Tenth,
                year_index: 57
            })
        } else if $a[58].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Ninth,
                earthly_branch: EarthlyBranch::Eleventh,
                year_index: 58
            })
        } else if $a[59].eq($v) {
            Some(LunarYear {
                heavenly_stems: HeavenlyStems::Tenth,
                earthly_branch: EarthlyBranch::Twelfth,
                year_index: 59
            })
        } else {
            None
        }
    };
}

impl LunarYear {
    /// 透過農曆年份字串來取得 `LunarYear` 實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarYear> {
        let s = s.as_ref();

        the_lunar_years_from!(THE_LUNAR_YEARS, s)
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
}

impl Display for LunarYear {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.to_str())
    }
}