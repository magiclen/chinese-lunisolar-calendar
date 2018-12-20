use super::{SolarYear, SolarMonth, SolarDate};

/// 判斷傳入的西曆年是否為閏年。(四年閏、四百年閏、百年不閏)
pub fn is_leap_solar_year<Y: Into<SolarYear>>(solar_year: Y) -> bool {
    let solar_year = solar_year.into();

    let year = solar_year.to_u16();

    ((year % 4 == 0) && (year % 100 != 0) || year % 400 == 0)
}

/// 計算傳入的西曆年共有幾天。
pub fn days_a_solar_year<Y: Into<SolarYear>>(solar_year: Y) -> u16 {
    if is_leap_solar_year(solar_year) {
        366
    } else {
        365
    }
}

/// 計算傳入的西曆月共有幾天。
pub fn days_in_a_solar_month<Y: Into<SolarYear>>(solar_year: Y, solar_month: SolarMonth) -> u8 {
    let month = solar_month.to_u8();

    let m = if month < 8 {
        1
    } else {
        0
    };

    if month % 2 == m {
        31
    } else if month == 2 {
        if is_leap_solar_year(solar_year) {
            29
        } else {
            28
        }
    } else {
        30
    }
}

/// 計算傳入的西曆年月日是該西曆年的第幾天。舉例：2013-01-04，就是第四天。
pub fn the_n_day_in_a_solar_year(solar_date: SolarDate) -> u16 {
    let mut n = 0;

    let solar_year = solar_date.get_solar_year();

    let month = solar_date.get_solar_month().to_u8();

    for i in 1..month {
        n += days_in_a_solar_month(solar_year, unsafe { SolarMonth::from_u8_unsafe(i) }) as u16;
    }

    n += solar_date.get_solar_day().to_u8() as u16;

    n
}