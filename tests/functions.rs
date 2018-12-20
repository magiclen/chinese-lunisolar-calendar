extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{SolarMonth, SolarDate};

#[test]
fn is_leap_solar_year() {
    assert_eq!(true, chinese_lunisolar_calendar::is_leap_solar_year(2000));
    assert_eq!(false, chinese_lunisolar_calendar::is_leap_solar_year(2001));
    assert_eq!(false, chinese_lunisolar_calendar::is_leap_solar_year(2002));
    assert_eq!(true, chinese_lunisolar_calendar::is_leap_solar_year(2004));
    assert_eq!(false, chinese_lunisolar_calendar::is_leap_solar_year(2100));
    assert_eq!(true, chinese_lunisolar_calendar::is_leap_solar_year(2104));
}

#[test]
fn days_a_solar_year() {
    assert_eq!(366, chinese_lunisolar_calendar::days_a_solar_year(2000));
    assert_eq!(365, chinese_lunisolar_calendar::days_a_solar_year(2001));
    assert_eq!(365, chinese_lunisolar_calendar::days_a_solar_year(2002));
    assert_eq!(366, chinese_lunisolar_calendar::days_a_solar_year(2004));
    assert_eq!(365, chinese_lunisolar_calendar::days_a_solar_year(2100));
    assert_eq!(366, chinese_lunisolar_calendar::days_a_solar_year(2104));
}

#[test]
fn days_in_a_solar_month() {
    assert_eq!(31, chinese_lunisolar_calendar::days_in_a_solar_month(2000, SolarMonth::January));
    assert_eq!(29, chinese_lunisolar_calendar::days_in_a_solar_month(2000, SolarMonth::February));
    assert_eq!(31, chinese_lunisolar_calendar::days_in_a_solar_month(2000, SolarMonth::March));
    assert_eq!(30, chinese_lunisolar_calendar::days_in_a_solar_month(2000, SolarMonth::April));
    assert_eq!(30, chinese_lunisolar_calendar::days_in_a_solar_month(2000, SolarMonth::November));
    assert_eq!(31, chinese_lunisolar_calendar::days_in_a_solar_month(2000, SolarMonth::December));
    assert_eq!(31, chinese_lunisolar_calendar::days_in_a_solar_month(2100, SolarMonth::January));
    assert_eq!(28, chinese_lunisolar_calendar::days_in_a_solar_month(2100, SolarMonth::February));
    assert_eq!(31, chinese_lunisolar_calendar::days_in_a_solar_month(2100, SolarMonth::March));
    assert_eq!(30, chinese_lunisolar_calendar::days_in_a_solar_month(2100, SolarMonth::April));
    assert_eq!(30, chinese_lunisolar_calendar::days_in_a_solar_month(2100, SolarMonth::November));
    assert_eq!(31, chinese_lunisolar_calendar::days_in_a_solar_month(2100, SolarMonth::December));
}

#[test]
fn the_n_day_in_a_solar_year() {
    assert_eq!(4, chinese_lunisolar_calendar::the_n_day_in_a_solar_year(SolarDate::from_ymd(2013, 1, 4).unwrap()));
    assert_eq!(63, chinese_lunisolar_calendar::the_n_day_in_a_solar_year(SolarDate::from_ymd(2013, 3, 4).unwrap()));
    assert_eq!(94, chinese_lunisolar_calendar::the_n_day_in_a_solar_year(SolarDate::from_ymd(2013, 4, 4).unwrap()));
    assert_eq!(124, chinese_lunisolar_calendar::the_n_day_in_a_solar_year(SolarDate::from_ymd(2013, 5, 4).unwrap()));
    assert_eq!(64, chinese_lunisolar_calendar::the_n_day_in_a_solar_year(SolarDate::from_ymd(2020, 3, 4).unwrap()));
    assert_eq!(95, chinese_lunisolar_calendar::the_n_day_in_a_solar_year(SolarDate::from_ymd(2020, 4, 4).unwrap()));
    assert_eq!(125, chinese_lunisolar_calendar::the_n_day_in_a_solar_year(SolarDate::from_ymd(2020, 5, 4).unwrap()));
}