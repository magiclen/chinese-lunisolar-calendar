extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::SolarYear;

#[test]
fn from_str_to_u16() {
    let solar_year = SolarYear::from_str("二零一八").unwrap();
    assert_eq!(2018, solar_year.to_u16());
}

#[test]
fn from_u16_to_chinese_string() {
    let solar_year = SolarYear::from_u16(2018);
    assert_eq!(solar_year.to_chinese_string(), "二零一八");
}