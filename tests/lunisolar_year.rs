extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{LunisolarYear, HeavenlyStems, EarthlyBranch, Zodiac, LunarYear, LunarMonth};

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
fn get_leap_lunar_month() {
    assert_eq!(Some(LunarMonth::LeapThird), LunisolarYear::from_solar_year(1993).unwrap().get_leap_lunar_month());
    assert_eq!(None, LunisolarYear::from_solar_year(2018).unwrap().get_leap_lunar_month());
    assert_eq!(None, LunisolarYear::from_solar_year(2019).unwrap().get_leap_lunar_month());
    assert_eq!(Some(LunarMonth::LeapFourth), LunisolarYear::from_solar_year(2020).unwrap().get_leap_lunar_month());
}

#[test]
fn get_total_days_in_leap_month() {
    assert_eq!(29, LunisolarYear::from_solar_year(1993).unwrap().get_total_days_in_leap_month());
    assert_eq!(0, LunisolarYear::from_solar_year(2018).unwrap().get_total_days_in_leap_month());
    assert_eq!(0, LunisolarYear::from_solar_year(2019).unwrap().get_total_days_in_leap_month());
    assert_eq!(29, LunisolarYear::from_solar_year(2020).unwrap().get_total_days_in_leap_month());
}

#[test]
fn get_total_days() {
    assert_eq!(383, LunisolarYear::from_solar_year(1993).unwrap().get_total_days());
    assert_eq!(354, LunisolarYear::from_solar_year(2018).unwrap().get_total_days());
    assert_eq!(354, LunisolarYear::from_solar_year(2019).unwrap().get_total_days());
    assert_eq!(384, LunisolarYear::from_solar_year(2020).unwrap().get_total_days());
    assert_eq!(355, LunisolarYear::from_solar_year(2022).unwrap().get_total_days());
}

#[test]
fn to_lunar_year() {
    assert_eq!(LunarYear::from_era(HeavenlyStems::Tenth, EarthlyBranch::Tenth), LunisolarYear::from_solar_year(1993).unwrap().to_lunar_year());
    assert_eq!(LunarYear::from_era(HeavenlyStems::Fifth, EarthlyBranch::Eleventh), LunisolarYear::from_solar_year(2018).unwrap().to_lunar_year());
}