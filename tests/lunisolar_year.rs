extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{LunisolarYear, HeavenlyStems, EarthlyBranch, Zodiac, LunarYear};

#[test]
fn get_heavenly_stems() {
    assert_eq!(HeavenlyStems::Tenth, LunisolarYear::from_solar_year(1993).unwrap().get_heavenly_stems());
    assert_eq!(HeavenlyStems::Fifth, LunisolarYear::from_solar_year(2018).unwrap().get_heavenly_stems());
}

#[test]
fn get_earthly_branch() {
    assert_eq!(EarthlyBranch::Tenth, LunisolarYear::from_solar_year(1993).unwrap().get_earthly_branch());
    assert_eq!(EarthlyBranch::Eleventh, LunisolarYear::from_solar_year(2018).unwrap().get_earthly_branch());
}

#[test]
fn get_zodiac() {
    assert_eq!(Zodiac::Rooster, LunisolarYear::from_solar_year(1993).unwrap().get_zodiac());
    assert_eq!(Zodiac::Dog, LunisolarYear::from_solar_year(2018).unwrap().get_zodiac());
}

#[test]
fn to_lunar_year() {
    assert_eq!(LunarYear::from_era(HeavenlyStems::Tenth, EarthlyBranch::Tenth), LunisolarYear::from_solar_year(1993).unwrap().to_lunar_year());
    assert_eq!(LunarYear::from_era(HeavenlyStems::Fifth, EarthlyBranch::Eleventh), LunisolarYear::from_solar_year(2018).unwrap().to_lunar_year());
}