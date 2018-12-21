extern crate chinese_lunisolar_calendar;

use std::ops::Add;

use chinese_lunisolar_calendar::{SolarDate, LunarDate, MIN_LUNAR_DATE_IN_SOLAR_CALENDAR, MAX_LUNAR_DATE_IN_SOLAR_CALENDAR};
use chinese_lunisolar_calendar::chrono::Duration;

#[test]
fn loopback() {
    let mut current = *MIN_LUNAR_DATE_IN_SOLAR_CALENDAR;

    while current <= *MAX_LUNAR_DATE_IN_SOLAR_CALENDAR {
        let lunar_date = LunarDate::from_naive_date(current).unwrap();
        let solar_date = SolarDate::from_naive_date(current).unwrap();

        assert_eq!(lunar_date, solar_date.to_lunar_date().unwrap());
        assert_eq!(solar_date, lunar_date.to_solar_date());

        current = current.add(Duration::days(1));
    }
}