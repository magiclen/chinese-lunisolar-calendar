extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::SolarDay;

#[test]
fn from_str() {
    assert_eq!(SolarDay::First, SolarDay::from_str("一").unwrap());
    assert_eq!(SolarDay::Second, SolarDay::from_str("二").unwrap());
    assert_eq!(SolarDay::Fifth, SolarDay::from_str("五").unwrap());
    assert_eq!(SolarDay::ThirtyFirst, SolarDay::from_str("三十一").unwrap());
}

#[test]
fn to_str() {
    assert_eq!(SolarDay::First.to_str(), "一");
    assert_eq!(SolarDay::Second.to_str(), "二");
    assert_eq!(SolarDay::Fifth.to_str(), "五");
    assert_eq!(SolarDay::ThirtyFirst.to_str(), "三十一");
}

#[test]
fn from_u8() {
    assert_eq!(SolarDay::First, SolarDay::from_u8(1).unwrap());
    assert_eq!(SolarDay::Second, SolarDay::from_u8(2).unwrap());
    assert_eq!(SolarDay::Fifth, SolarDay::from_u8(5).unwrap());
    assert_eq!(SolarDay::ThirtyFirst, SolarDay::from_u8(31).unwrap());
}

#[test]
fn to_u8() {
    assert_eq!(SolarDay::First.to_u8(), 1);
    assert_eq!(SolarDay::Second.to_u8(), 2);
    assert_eq!(SolarDay::Fifth.to_u8(), 5);
    assert_eq!(SolarDay::ThirtyFirst.to_u8(), 31);
}