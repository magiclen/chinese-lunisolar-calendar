use chinese_lunisolar_calendar::{EarthlyBranch, HeavenlyStems, LunarYear};

#[test]
fn from_str() {
    let lunar_year = LunarYear::from_str("甲午").unwrap();
    assert_eq!(HeavenlyStems::First, lunar_year.get_heavenly_stems());
    assert_eq!(EarthlyBranch::Seventh, lunar_year.get_earthly_branch());
}

#[test]
fn to_str() {
    let lunar_year = LunarYear::from_era(HeavenlyStems::First, EarthlyBranch::First);
    assert_eq!(lunar_year.to_str(), "甲子");

    let lunar_year = LunarYear::from_era(HeavenlyStems::First, EarthlyBranch::Eleventh);
    assert_eq!(lunar_year.to_str(), "甲戌");

    let lunar_year = LunarYear::from_era(HeavenlyStems::Third, EarthlyBranch::First);
    assert_eq!(lunar_year.to_str(), "丙子");

    let lunar_year = LunarYear::from_era(HeavenlyStems::First, EarthlyBranch::Ninth);
    assert_eq!(lunar_year.to_str(), "甲申");

    let lunar_year = LunarYear::from_era(HeavenlyStems::Fifth, EarthlyBranch::First);
    assert_eq!(lunar_year.to_str(), "戊子");

    let lunar_year = LunarYear::from_era(HeavenlyStems::First, EarthlyBranch::Seventh);
    assert_eq!(lunar_year.to_str(), "甲午");

    let lunar_year = LunarYear::from_era(HeavenlyStems::Sixth, EarthlyBranch::Twelfth);
    assert_eq!(lunar_year.to_str(), "己亥");

    let lunar_year = LunarYear::from_era(HeavenlyStems::Seventh, EarthlyBranch::First);
    assert_eq!(lunar_year.to_str(), "庚子");

    let lunar_year = LunarYear::from_era(HeavenlyStems::First, EarthlyBranch::Fifth);
    assert_eq!(lunar_year.to_str(), "甲辰");

    let lunar_year = LunarYear::from_era(HeavenlyStems::Ninth, EarthlyBranch::First);
    assert_eq!(lunar_year.to_str(), "壬子");

    let lunar_year = LunarYear::from_era(HeavenlyStems::First, EarthlyBranch::Third);
    assert_eq!(lunar_year.to_str(), "甲寅");
}
