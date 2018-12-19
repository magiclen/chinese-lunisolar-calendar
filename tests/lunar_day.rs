extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::LunarDay;

#[test]
fn test_from_str() {
    assert_eq!(LunarDay::First, LunarDay::from_str("初一").unwrap());
    assert_eq!(LunarDay::Second, LunarDay::from_str("初二").unwrap());
    assert_eq!(LunarDay::Fifth, LunarDay::from_str("初五").unwrap());
    assert_eq!(LunarDay::Thirty, LunarDay::from_str("三十").unwrap());
}

#[test]
fn test_to_str() {
    assert_eq!(LunarDay::First.to_str(), "初一");
    assert_eq!(LunarDay::Second.to_str(), "初二");
    assert_eq!(LunarDay::Fifth.to_str(), "初五");
    assert_eq!(LunarDay::Thirty.to_str(), "三十");
}

#[test]
fn test_from_u8() {
    assert_eq!(LunarDay::First, LunarDay::from_u8(1).unwrap());
    assert_eq!(LunarDay::Second, LunarDay::from_u8(2).unwrap());
    assert_eq!(LunarDay::Fifth, LunarDay::from_u8(5).unwrap());
    assert_eq!(LunarDay::Thirty, LunarDay::from_u8(30).unwrap());
}

#[test]
fn test_to_u8() {
    assert_eq!(LunarDay::First.to_u8(), 1);
    assert_eq!(LunarDay::Second.to_u8(), 2);
    assert_eq!(LunarDay::Fifth.to_u8(), 5);
    assert_eq!(LunarDay::Thirty.to_u8(), 30);
}