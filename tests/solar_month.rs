extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::SolarMonth;

#[test]
fn from_str() {
    assert_eq!(SolarMonth::January, SolarMonth::from_str("一月").unwrap());
    assert_eq!(SolarMonth::February, SolarMonth::from_str("二月").unwrap());
    assert_eq!(SolarMonth::May, SolarMonth::from_str("五月").unwrap());
    assert_eq!(SolarMonth::December, SolarMonth::from_str("十二月").unwrap());
}

#[test]
fn to_str() {
    assert_eq!(SolarMonth::January.to_str(), "一月");
    assert_eq!(SolarMonth::February.to_str(), "二月");
    assert_eq!(SolarMonth::May.to_str(), "五月");
    assert_eq!(SolarMonth::December.to_str(), "十二月");
}

#[test]
fn from_u8() {
    assert_eq!(SolarMonth::January, SolarMonth::from_u8(1).unwrap());
    assert_eq!(SolarMonth::February, SolarMonth::from_u8(2).unwrap());
    assert_eq!(SolarMonth::May, SolarMonth::from_u8(5).unwrap());
    assert_eq!(SolarMonth::December, SolarMonth::from_u8(12).unwrap());
}

#[test]
fn to_u8() {
    assert_eq!(SolarMonth::January.to_u8(), 1);
    assert_eq!(SolarMonth::February.to_u8(), 2);
    assert_eq!(SolarMonth::May.to_u8(), 5);
    assert_eq!(SolarMonth::December.to_u8(), 12);
}

#[test]
fn get_total_days() {
    assert_eq!(31, SolarMonth::January.get_total_days(2000));
    assert_eq!(29, SolarMonth::February.get_total_days(2000));
    assert_eq!(31, SolarMonth::March.get_total_days(2000));
    assert_eq!(30, SolarMonth::April.get_total_days(2000));
    assert_eq!(30, SolarMonth::November.get_total_days(2000));
    assert_eq!(31, SolarMonth::December.get_total_days(2000));
    assert_eq!(31, SolarMonth::January.get_total_days(2100));
    assert_eq!(28, SolarMonth::February.get_total_days(2100));
    assert_eq!(31, SolarMonth::March.get_total_days(2100));
    assert_eq!(30, SolarMonth::April.get_total_days(2100));
    assert_eq!(30, SolarMonth::November.get_total_days(2100));
    assert_eq!(31, SolarMonth::December.get_total_days(2100));
}