extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::SolarMonth;

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