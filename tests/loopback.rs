use std::ops::Add;

use chinese_lunisolar_calendar::{
    chrono::Duration, LunisolarDate, SolarDate, MAX_LUNAR_DATE_IN_SOLAR_CALENDAR,
    MIN_LUNAR_DATE_IN_SOLAR_CALENDAR,
};

#[test]
fn loopback() {
    let mut current = *MIN_LUNAR_DATE_IN_SOLAR_CALENDAR;

    while current <= *MAX_LUNAR_DATE_IN_SOLAR_CALENDAR {
        let lunar_date = LunisolarDate::from_naive_date(current).unwrap();
        let solar_date = SolarDate::from_naive_date(current).unwrap();

        assert_eq!(lunar_date, solar_date.to_lunisolar_date().unwrap());
        assert_eq!(solar_date, lunar_date.to_solar_date());

        current = current.add(Duration::days(1));
    }
}
