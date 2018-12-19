use super::{ChineseVariant, HeavenlyStems, EarthlyBranch, Zodiac};

/// 農曆年份由天干加地支組成，六十年一輪。
#[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunarYear {
    /// 天干
    pub(crate) heavenly_stems: HeavenlyStems,
    /// 地支
    pub(crate) earthly_branch: EarthlyBranch
}

impl LunarYear {
    /// 透過農曆年份字串來取得 `LunarYear` 列舉實體。
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<LunarYear> {
        let mut chars = s.as_ref().chars();

        let hs = chars.next();

        match hs {
            Some(hs) => {
                let heavenly_stems = HeavenlyStems::from_char(hs);

                match heavenly_stems {
                    Some(heavenly_stems) => {
                        let eb = chars.next();

                        match eb {
                            Some(eb) => {
                                let earthly_branch = EarthlyBranch::from_char(eb);

                                match earthly_branch {
                                    Some(earthly_branch) => {
                                        if let Some(_) = chars.next() {
                                            None
                                        } else {
                                            Some(LunarYear {
                                                heavenly_stems,
                                                earthly_branch,
                                            })
                                        }
                                    }
                                    None => None
                                }
                            }
                            None => None
                        }
                    }
                    None => None
                }
            }
            None => None
        }
    }

    /// 取得 `LunarYear` 列舉實體所代表的農曆年份字串。
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(6);

        self.write_to_string(&mut s);

        s
    }

    /// 取得 `LunarYear` 列舉實體所代表的農曆年份字串。
    pub fn write_to_string(&self, s: &mut String) {
        s.push_str(self.heavenly_stems.to_str());
        s.push_str(self.earthly_branch.to_str());
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