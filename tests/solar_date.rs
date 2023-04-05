#![allow(deprecated)] // wait for chrono 0.5

use chinese_lunisolar_calendar::{
    chrono::prelude::*, LunisolarDate, SolarDate, SolarDay, SolarMonth, SolarYear,
};

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
    let solar_date = SolarDate::from_solar_year_month_day(
        2018,
        SolarMonth::from_u8(6).unwrap(),
        SolarDay::from_u8(19).unwrap(),
    )
    .unwrap();

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
fn from_lunisolar_date() {
    let lunisolar_date = LunisolarDate::from_ymd(1993, 6, false, 23).unwrap();

    let solar_date = SolarDate::from_lunisolar_date(lunisolar_date);

    assert_eq!(SolarYear::from_u16(1993), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(8).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(10).unwrap(), solar_date.get_solar_day());
}

#[test]
fn from_str() {
    let solar_date = SolarDate::from_str("二零一八年六月十九日").unwrap();

    assert_eq!(SolarYear::from_u16(2018), solar_date.get_solar_year());
    assert_eq!(SolarMonth::from_u8(6).unwrap(), solar_date.get_solar_month());
    assert_eq!(SolarDay::from_u8(19).unwrap(), solar_date.get_solar_day());

    let solar_date = SolarDate::from_str("二〇一八年六月十九日").unwrap();

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
    assert_eq!("二〇一八年六月十九日", solar_date.to_chinese_string_2());
}

#[test]
fn the_n_day_in_this_year() {
    assert_eq!(4, SolarDate::from_ymd(2013, 1, 4).unwrap().the_n_day_in_this_year());
    assert_eq!(63, SolarDate::from_ymd(2013, 3, 4).unwrap().the_n_day_in_this_year());
    assert_eq!(94, SolarDate::from_ymd(2013, 4, 4).unwrap().the_n_day_in_this_year());
    assert_eq!(124, SolarDate::from_ymd(2013, 5, 4).unwrap().the_n_day_in_this_year());
    assert_eq!(64, SolarDate::from_ymd(2020, 3, 4).unwrap().the_n_day_in_this_year());
    assert_eq!(95, SolarDate::from_ymd(2020, 4, 4).unwrap().the_n_day_in_this_year());
    assert_eq!(125, SolarDate::from_ymd(2020, 5, 4).unwrap().the_n_day_in_this_year());
}
