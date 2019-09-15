extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{ChineseVariant, LunarMonth, LunisolarYear};

#[test]
fn from_str() {
    assert_eq!(LunarMonth::First, LunarMonth::from_str("正月").unwrap());
    assert_eq!(LunarMonth::Second, LunarMonth::from_str("二月").unwrap());
    assert_eq!(LunarMonth::Fifth, LunarMonth::from_str("五月").unwrap());
    assert_eq!(LunarMonth::Sixth, LunarMonth::from_str("六月").unwrap());
    assert_eq!(LunarMonth::Twelfth, LunarMonth::from_str("臘月").unwrap());
    assert_eq!(LunarMonth::Twelfth, LunarMonth::from_str("腊月").unwrap());
    assert_eq!(LunarMonth::LeapFirst, LunarMonth::from_str("閏一月").unwrap());
    assert_eq!(LunarMonth::LeapSecond, LunarMonth::from_str("閏二月").unwrap());
    assert_eq!(LunarMonth::LeapFifth, LunarMonth::from_str("閏五月").unwrap());
    assert_eq!(LunarMonth::LeapSixth, LunarMonth::from_str("閏六月").unwrap());
    assert_eq!(LunarMonth::LeapTwelfth, LunarMonth::from_str("閏臘月").unwrap());
    assert_eq!(LunarMonth::LeapTwelfth, LunarMonth::from_str("闰腊月").unwrap());
}

#[test]
fn to_str() {
    assert_eq!(LunarMonth::First.to_str(ChineseVariant::Traditional), "正月");
    assert_eq!(LunarMonth::Second.to_str(ChineseVariant::Traditional), "二月");
    assert_eq!(LunarMonth::Fifth.to_str(ChineseVariant::Traditional), "五月");
    assert_eq!(LunarMonth::Sixth.to_str(ChineseVariant::Traditional), "六月");
    assert_eq!(LunarMonth::Twelfth.to_str(ChineseVariant::Traditional), "臘月");
    assert_eq!(LunarMonth::Twelfth.to_str(ChineseVariant::Simple), "腊月");
    assert_eq!(LunarMonth::LeapFirst.to_str(ChineseVariant::Traditional), "閏正月");
    assert_eq!(LunarMonth::LeapSecond.to_str(ChineseVariant::Traditional), "閏二月");
    assert_eq!(LunarMonth::LeapFifth.to_str(ChineseVariant::Traditional), "閏五月");
    assert_eq!(LunarMonth::LeapSixth.to_str(ChineseVariant::Traditional), "閏六月");
    assert_eq!(LunarMonth::LeapTwelfth.to_str(ChineseVariant::Traditional), "閏臘月");
    assert_eq!(LunarMonth::LeapTwelfth.to_str(ChineseVariant::Simple), "闰腊月");
}

#[test]
fn from_u8() {
    assert_eq!(LunarMonth::First, LunarMonth::from_u8(1, false).unwrap());
    assert_eq!(LunarMonth::Second, LunarMonth::from_u8(2, false).unwrap());
    assert_eq!(LunarMonth::Fifth, LunarMonth::from_u8(5, false).unwrap());
    assert_eq!(LunarMonth::Sixth, LunarMonth::from_u8(6, false).unwrap());
    assert_eq!(LunarMonth::Twelfth, LunarMonth::from_u8(12, false).unwrap());
    assert_eq!(LunarMonth::LeapFirst, LunarMonth::from_u8(1, true).unwrap());
    assert_eq!(LunarMonth::LeapSecond, LunarMonth::from_u8(2, true).unwrap());
    assert_eq!(LunarMonth::LeapFifth, LunarMonth::from_u8(5, true).unwrap());
    assert_eq!(LunarMonth::LeapSixth, LunarMonth::from_u8(6, true).unwrap());
    assert_eq!(LunarMonth::LeapTwelfth, LunarMonth::from_u8(12, true).unwrap());
}

#[test]
fn to_u8() {
    assert_eq!(LunarMonth::First.to_u8(), 1);
    assert_eq!(LunarMonth::Second.to_u8(), 2);
    assert_eq!(LunarMonth::Fifth.to_u8(), 5);
    assert_eq!(LunarMonth::Sixth.to_u8(), 6);
    assert_eq!(LunarMonth::Twelfth.to_u8(), 12);
    assert_eq!(LunarMonth::LeapFirst.to_u8(), 1);
    assert_eq!(LunarMonth::LeapSecond.to_u8(), 2);
    assert_eq!(LunarMonth::LeapFifth.to_u8(), 5);
    assert_eq!(LunarMonth::LeapSixth.to_u8(), 6);
    assert_eq!(LunarMonth::LeapTwelfth.to_u8(), 12);
}

#[test]
fn get_total_days() {
    assert_eq!(
        Some(30),
        LunarMonth::Fourth.get_total_days(LunisolarYear::from_solar_year(2020).unwrap())
    );
    assert_eq!(
        Some(29),
        LunarMonth::LeapFourth.get_total_days(LunisolarYear::from_solar_year(2020).unwrap())
    );
    assert_eq!(
        Some(30),
        LunarMonth::Fifth.get_total_days(LunisolarYear::from_solar_year(2020).unwrap())
    );
}
