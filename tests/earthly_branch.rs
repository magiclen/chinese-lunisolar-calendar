#![allow(deprecated)] // wait for chrono 0.5

use chinese_lunisolar_calendar::{EarthlyBranch, Zodiac};

use chinese_lunisolar_calendar::chrono::prelude::*;

#[test]
fn from_str() {
    assert_eq!(EarthlyBranch::First, EarthlyBranch::from_str("子").unwrap());
    assert_eq!(EarthlyBranch::Second, EarthlyBranch::from_str("丑").unwrap());
    assert_eq!(EarthlyBranch::Fifth, EarthlyBranch::from_str("辰").unwrap());
    assert_eq!(EarthlyBranch::Twelfth, EarthlyBranch::from_str("亥").unwrap());
}

#[test]
fn from_time() {
    assert_eq!(EarthlyBranch::First, EarthlyBranch::from_time(NaiveTime::from_hms(23, 30, 0)));
    assert_eq!(EarthlyBranch::Second, EarthlyBranch::from_time(NaiveTime::from_hms(1, 30, 0)));
    assert_eq!(EarthlyBranch::Fifth, EarthlyBranch::from_time(NaiveTime::from_hms(7, 0, 1)));
    assert_eq!(EarthlyBranch::Twelfth, EarthlyBranch::from_time(NaiveTime::from_hms(22, 0, 0)));
}

#[test]
fn to_str() {
    assert_eq!(EarthlyBranch::First.to_str(), "子");
    assert_eq!(EarthlyBranch::Second.to_str(), "丑");
    assert_eq!(EarthlyBranch::Fifth.to_str(), "辰");
    assert_eq!(EarthlyBranch::Twelfth.to_str(), "亥");
}

#[test]
fn from_char() {
    assert_eq!(EarthlyBranch::First, EarthlyBranch::from_char('子').unwrap());
    assert_eq!(EarthlyBranch::Second, EarthlyBranch::from_char('丑').unwrap());
    assert_eq!(EarthlyBranch::Fifth, EarthlyBranch::from_char('辰').unwrap());
    assert_eq!(EarthlyBranch::Twelfth, EarthlyBranch::from_char('亥').unwrap());
}

#[test]
fn to_char() {
    assert_eq!(EarthlyBranch::First.to_char(), '子');
    assert_eq!(EarthlyBranch::Second.to_char(), '丑');
    assert_eq!(EarthlyBranch::Fifth.to_char(), '辰');
    assert_eq!(EarthlyBranch::Twelfth.to_char(), '亥');
}

#[test]
fn from_zodiac() {
    assert_eq!(EarthlyBranch::from_zodiac(Zodiac::Rat), EarthlyBranch::First);
    assert_eq!(EarthlyBranch::from_zodiac(Zodiac::Ox), EarthlyBranch::Second);
    assert_eq!(EarthlyBranch::from_zodiac(Zodiac::Dragon), EarthlyBranch::Fifth);
    assert_eq!(EarthlyBranch::from_zodiac(Zodiac::Pig), EarthlyBranch::Twelfth);
}

#[test]
fn to_zodiac() {
    assert_eq!(Zodiac::Rat, EarthlyBranch::First.to_zodiac());
    assert_eq!(Zodiac::Ox, EarthlyBranch::Second.to_zodiac());
    assert_eq!(Zodiac::Dragon, EarthlyBranch::Fifth.to_zodiac());
    assert_eq!(Zodiac::Pig, EarthlyBranch::Twelfth.to_zodiac());
}
