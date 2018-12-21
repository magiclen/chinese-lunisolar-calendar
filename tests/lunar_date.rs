extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{ChineseVariant, SolarYear, SolarDate, LunisolarYear, LunarYear, LunarMonth, LunarDay, LunarDate};

use chinese_lunisolar_calendar::chrono::prelude::*;

#[test]
fn from_solar_date() {
    let solar_date = SolarDate::from_ymd(1993, 1, 12).unwrap();

    let lunar_date = LunarDate::from_solar_date(solar_date).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1992).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("壬申").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(12, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(20).unwrap(), lunar_date.get_lunar_day());

    let solar_date = SolarDate::from_ymd(1993, 1, 23).unwrap();

    let lunar_date = LunarDate::from_solar_date(solar_date).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(1, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(1).unwrap(), lunar_date.get_lunar_day());

    let solar_date = SolarDate::from_ymd(1993, 4, 22).unwrap();

    let lunar_date = LunarDate::from_solar_date(solar_date).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(3, true).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(1).unwrap(), lunar_date.get_lunar_day());

    let solar_date = SolarDate::from_ymd(1993, 8, 10).unwrap();

    let lunar_date = LunarDate::from_solar_date(solar_date).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(6, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(23).unwrap(), lunar_date.get_lunar_day());

    let solar_date = SolarDate::from_ymd(1993, 12, 12).unwrap();

    let lunar_date = LunarDate::from_solar_date(solar_date).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(10, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(29).unwrap(), lunar_date.get_lunar_day());
}

#[test]
fn from_ymd() {
    let lunar_date = LunarDate::from_ymd(1992, 12, false, 20).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1992).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("壬申").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(12, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(20).unwrap(), lunar_date.get_lunar_day());

    let lunar_date = LunarDate::from_ymd(1993, 1, false, 1).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(1, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(1).unwrap(), lunar_date.get_lunar_day());

    let lunar_date = LunarDate::from_ymd(1993, 3, true, 1).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(3, true).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(1).unwrap(), lunar_date.get_lunar_day());

    let lunar_date = LunarDate::from_ymd(1993, 6, false, 23).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(6, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(23).unwrap(), lunar_date.get_lunar_day());

    let lunar_date = LunarDate::from_ymd(1993, 10, false, 29).unwrap();

    assert_eq!(SolarYear::from_u16(1993), lunar_date.get_solar_year());
    assert_eq!(LunisolarYear::from_solar_year(1993).unwrap(), lunar_date.get_lunisolar_year());
    assert_eq!(LunarYear::from_str("癸酉").unwrap(), lunar_date.get_lunar_year());
    assert_eq!(LunarMonth::from_u8(10, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(29).unwrap(), lunar_date.get_lunar_day());
}

#[test]
fn from_str() {
    let lunar_date = LunarDate::from_str("二零一八　戊戌、狗年　六月　十九").unwrap();

    assert_eq!(SolarYear::from_u16(2018), lunar_date.get_solar_year());
    assert_eq!(LunarMonth::from_u8(6, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(19).unwrap(), lunar_date.get_lunar_day());

    let lunar_date = LunarDate::from_str("2018　六月　十九日").unwrap();

    assert_eq!(SolarYear::from_u16(2018), lunar_date.get_solar_year());
    assert_eq!(LunarMonth::from_u8(6, false).unwrap(), lunar_date.get_lunar_month());
    assert_eq!(LunarDay::from_u8(19).unwrap(), lunar_date.get_lunar_day());
}

#[test]
fn to_chinese_string() {
    let lunar_date = LunarDate::from_ymd(2018, 6,false, 19).unwrap();

    assert_eq!("二零一八　戊戌、狗年　六月　十九", lunar_date.to_chinese_string(ChineseVariant::Traditional));
}

#[test]
fn the_n_day_in_this_year() {
    assert_eq!(344, LunarDate::from_ymd(1992, 12, false, 20).unwrap().the_n_day_in_this_year());
    assert_eq!(1, LunarDate::from_ymd(1993, 1, false, 1).unwrap().the_n_day_in_this_year());
    assert_eq!(60, LunarDate::from_ymd(1993, 3, false, 1).unwrap().the_n_day_in_this_year());
    assert_eq!(200, LunarDate::from_ymd(1993, 6, false, 23).unwrap().the_n_day_in_this_year());
    assert_eq!(324, LunarDate::from_ymd(1993, 10, false, 29).unwrap().the_n_day_in_this_year());
}



#[test]
fn get_ba_zi_weight_by_time() {
    assert_eq!(4.8,  LunarDate::from_ymd(1993, 6, false, 23).unwrap().get_ba_zi_weight_by_time(NaiveTime::from_hms(23,30,0)));
}

