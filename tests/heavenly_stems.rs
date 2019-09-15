extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::HeavenlyStems;

#[test]
fn from_str() {
    assert_eq!(HeavenlyStems::First, HeavenlyStems::from_str("甲").unwrap());
    assert_eq!(HeavenlyStems::Second, HeavenlyStems::from_str("乙").unwrap());
    assert_eq!(HeavenlyStems::Fifth, HeavenlyStems::from_str("戊").unwrap());
    assert_eq!(HeavenlyStems::Tenth, HeavenlyStems::from_str("癸").unwrap());
}

#[test]
fn to_str() {
    assert_eq!(HeavenlyStems::First.to_str(), "甲");
    assert_eq!(HeavenlyStems::Second.to_str(), "乙");
    assert_eq!(HeavenlyStems::Fifth.to_str(), "戊");
    assert_eq!(HeavenlyStems::Tenth.to_str(), "癸");
}

#[test]
fn from_char() {
    assert_eq!(HeavenlyStems::First, HeavenlyStems::from_char('甲').unwrap());
    assert_eq!(HeavenlyStems::Second, HeavenlyStems::from_char('乙').unwrap());
    assert_eq!(HeavenlyStems::Fifth, HeavenlyStems::from_char('戊').unwrap());
    assert_eq!(HeavenlyStems::Tenth, HeavenlyStems::from_char('癸').unwrap());
}

#[test]
fn to_char() {
    assert_eq!(HeavenlyStems::First.to_char(), '甲');
    assert_eq!(HeavenlyStems::Second.to_char(), '乙');
    assert_eq!(HeavenlyStems::Fifth.to_char(), '戊');
    assert_eq!(HeavenlyStems::Tenth.to_char(), '癸');
}
