extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{EarthlyBranch, HeavenlyStems, LunarYear};

#[test]
fn from_str() {
    let lunar_year = LunarYear::from_str("甲午").unwrap();
    assert_eq!(HeavenlyStems::First, lunar_year.get_heavenly_stems());
    assert_eq!(EarthlyBranch::Seventh, lunar_year.get_earthly_branch());
}

#[test]
fn to_str() {
    let lunar_year = LunarYear::from_era(HeavenlyStems::First, EarthlyBranch::Seventh);
    assert_eq!(lunar_year.to_str(), "甲午");
}
