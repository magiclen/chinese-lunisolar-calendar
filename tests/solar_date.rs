extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{SolarDate, SolarYear, SolarMonth, SolarDay};
use chinese_lunisolar_calendar::chrono::prelude::*;

#[test]
fn from_naive_date() {
    let naive_date = NaiveDate::from_ymd(2018, 6, 19);

    let solar_date = SolarDate::from_naive_date(naive_date).unwrap();

    assert_eq!(SolarYear::from_u16(2018), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(6).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(19).unwrap(), solar_date.get_solar_day());
}


#[test]
fn from_date() {
    let date: Date<Utc> = Date::from_utc(NaiveDate::from_ymd(2018, 6, 19), Utc);

    let solar_date = SolarDate::from_date(date).unwrap();

    assert_eq!(SolarYear::from_u16(2018), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(6).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(19).unwrap(), solar_date.get_solar_day());
}

#[test]
fn from_solar_year_month_day() {
    let solar_date = SolarDate::from_solar_year_month_day(2018, SolarMonth::from_u8(6).unwrap(), SolarDay::from_u8(19).unwrap()).unwrap();

    assert_eq!(SolarYear::from_u16(2018), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(6).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(19).unwrap(), solar_date.get_solar_day());
}

#[test]
fn from_ymd() {
    let solar_date = SolarDate::from_ymd(2018, 6, 19).unwrap();

    assert_eq!(SolarYear::from_u16(2018), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(6).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(19).unwrap(), solar_date.get_solar_day());
}

#[test]
fn from_str() {
    let solar_date = SolarDate::from_str("二零一八年六月十九日").unwrap();

    assert_eq!(SolarYear::from_u16(2018), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(6).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(19).unwrap(), solar_date.get_solar_day());

    let solar_date = SolarDate::from_str("2018　六月　十九").unwrap();

    assert_eq!(SolarYear::from_u16(2018), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(6).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(19).unwrap(), solar_date.get_solar_day());
}

#[test]
fn to_chinese_string() {
    let solar_date = SolarDate::from_ymd(2018, 6, 19).unwrap();

    assert_eq!("二零一八年六月十九日", solar_date.to_chinese_string());
}

#[test]
fn to_string() {
    let solar_date = SolarDate::from_ymd(2018, 6, 19).unwrap();

    assert_eq!("2018-06-19", solar_date.to_string());
}
