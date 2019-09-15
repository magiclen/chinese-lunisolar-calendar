extern crate chinese_lunisolar_calendar;

use chinese_lunisolar_calendar::{ChineseVariant, EarthlyBranch, Zodiac};

#[test]
fn from_str() {
    assert_eq!(Zodiac::Rat, Zodiac::from_str("鼠").unwrap());
    assert_eq!(Zodiac::Ox, Zodiac::from_str("牛").unwrap());
    assert_eq!(Zodiac::Dragon, Zodiac::from_str("龍").unwrap());
    assert_eq!(Zodiac::Dragon, Zodiac::from_str("龙").unwrap());
    assert_eq!(Zodiac::Pig, Zodiac::from_str("豬").unwrap());
    assert_eq!(Zodiac::Pig, Zodiac::from_str("猪").unwrap());
}

#[test]
fn to_str() {
    assert_eq!(Zodiac::Rat.to_str(ChineseVariant::Traditional), "鼠");
    assert_eq!(Zodiac::Ox.to_str(ChineseVariant::Traditional), "牛");
    assert_eq!(Zodiac::Dragon.to_str(ChineseVariant::Traditional), "龍");
    assert_eq!(Zodiac::Dragon.to_str(ChineseVariant::Simple), "龙");
    assert_eq!(Zodiac::Pig.to_str(ChineseVariant::Traditional), "豬");
    assert_eq!(Zodiac::Pig.to_str(ChineseVariant::Simple), "猪");
}

#[test]
fn from_char() {
    assert_eq!(Zodiac::Rat, Zodiac::from_char('鼠').unwrap());
    assert_eq!(Zodiac::Ox, Zodiac::from_char('牛').unwrap());
    assert_eq!(Zodiac::Dragon, Zodiac::from_char('龍').unwrap());
    assert_eq!(Zodiac::Dragon, Zodiac::from_char('龙').unwrap());
    assert_eq!(Zodiac::Pig, Zodiac::from_char('豬').unwrap());
    assert_eq!(Zodiac::Pig, Zodiac::from_char('猪').unwrap());
}

#[test]
fn to_char() {
    assert_eq!(Zodiac::Rat.to_char(ChineseVariant::Traditional), '鼠');
    assert_eq!(Zodiac::Ox.to_char(ChineseVariant::Traditional), '牛');
    assert_eq!(Zodiac::Dragon.to_char(ChineseVariant::Traditional), '龍');
    assert_eq!(Zodiac::Dragon.to_char(ChineseVariant::Simple), '龙');
    assert_eq!(Zodiac::Pig.to_char(ChineseVariant::Traditional), '豬');
    assert_eq!(Zodiac::Pig.to_char(ChineseVariant::Simple), '猪');
}

#[test]
fn from_earthly_branch() {
    assert_eq!(Zodiac::from_earthly_branch(EarthlyBranch::First), Zodiac::Rat);
    assert_eq!(Zodiac::from_earthly_branch(EarthlyBranch::Second), Zodiac::Ox);
    assert_eq!(Zodiac::from_earthly_branch(EarthlyBranch::Fifth), Zodiac::Dragon);
    assert_eq!(Zodiac::from_earthly_branch(EarthlyBranch::Twelfth), Zodiac::Pig);
}

#[test]
fn to_earthly_branch() {
    assert_eq!(EarthlyBranch::First, Zodiac::Rat.to_earthly_branch());
    assert_eq!(EarthlyBranch::Second, Zodiac::Ox.to_earthly_branch());
    assert_eq!(EarthlyBranch::Fifth, Zodiac::Dragon.to_earthly_branch());
    assert_eq!(EarthlyBranch::Twelfth, Zodiac::Pig.to_earthly_branch());
}
