use super::{
    ChineseVariant, EarthlyBranch, LunarDay, LunarMonth, LunarYear, LunisolarError, LunisolarYear,
    SolarDate, SolarYear, MAX_LUNAR_DATE_IN_SOLAR_CALENDAR,
    MAX_LUNAR_DATE_IN_SOLAR_CALENDAR_NEW_YEAR_DIFFERENCE, MAX_YEAR_IN_SOLAR_CALENDAR,
    MIN_LUNAR_DATE_IN_SOLAR_CALENDAR, MIN_YEAR_IN_SOLAR_CALENDAR, NEW_YEAR_DIFFERENCE,
};

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use chrono::prelude::*;
use chrono::NaiveDate;

/// 農曆年月日，必須包含西曆年。
#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct LunisolarDate {
    solar_year: SolarYear,
    lunisolar_year: LunisolarYear,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
}

impl LunisolarDate {
    /// 將西曆年月日轉成農曆年月日(包含西曆年)。
    pub(crate) fn from_solar_date_inner(
        solar_date: SolarDate,
        naive_date: NaiveDate,
    ) -> Result<LunisolarDate, LunisolarError> {
        if naive_date < *MIN_LUNAR_DATE_IN_SOLAR_CALENDAR
            || naive_date > *MAX_LUNAR_DATE_IN_SOLAR_CALENDAR
        {
            Err(LunisolarError::OutOfLunarRange)
        } else {
            let solar_year = solar_date.get_solar_year();

            let year = solar_year.to_u16();

            let lunisolar_year = LunisolarYear::from_solar_year(solar_year).unwrap();

            let mut day_diff = solar_date.the_n_day_in_this_year() - 1;

            let (leap_month, leap_days, new_year_diff) = if year == MAX_YEAR_IN_SOLAR_CALENDAR {
                (0, 0, *MAX_LUNAR_DATE_IN_SOLAR_CALENDAR_NEW_YEAR_DIFFERENCE)
            } else {
                let new_year_diff =
                    u16::from(NEW_YEAR_DIFFERENCE[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize]);

                let leap_lunar_month = lunisolar_year.get_leap_lunar_month();

                match leap_lunar_month {
                    Some(leap_lunar_month) => {
                        (
                            leap_lunar_month.to_u8(),
                            lunisolar_year.get_total_days_in_leap_month_inner(leap_lunar_month),
                            new_year_diff,
                        )
                    }
                    None => (0, 0, new_year_diff),
                }
            };

            if day_diff < new_year_diff {
                // 若天數差距比「西曆新年與對應農曆年新年」之天數差距小，表示此西曆日期尚未進入下一個農曆年(尚未到達正月，還在農曆年尾)。
                let lunisolar_year = unsafe { LunisolarYear::from_solar_year_unsafe(year - 1) };

                let leap_lunar_month = lunisolar_year.get_leap_lunar_month();

                let (leap_month, leap_days) = match leap_lunar_month {
                    Some(leap_lunar_month) => {
                        (
                            leap_lunar_month.to_u8(),
                            lunisolar_year.get_total_days_in_leap_month_inner(leap_lunar_month),
                        )
                    }
                    None => (0, 0),
                };

                let mut day_diff = new_year_diff - day_diff; // 此時天數差距為此西曆日期到該西曆年應該對應的農曆年新年之差距天數(若西曆日期為2/3，農曆新年對應的西曆日期為2/10，則兩天數差距為40-(30+3)=7)。

                let mut is_leap = false;

                let mut month = 12;

                loop {
                    if month == leap_month {
                        if day_diff > leap_days {
                            day_diff -= leap_days;
                        } else {
                            is_leap = true;
                        }
                    }

                    let month_days = u16::from(
                        lunisolar_year
                            .get_total_days_in_a_month(unsafe {
                                LunarMonth::from_u8_unsafe(month, false)
                            })
                            .unwrap(),
                    );

                    if day_diff <= month_days {
                        break;
                    }

                    day_diff -= month_days;
                    is_leap = false;
                    month -= 1;
                }

                if day_diff == 0 {
                    Ok(LunisolarDate {
                        solar_year,
                        lunisolar_year,
                        lunar_month: unsafe {
                            LunarMonth::from_u8_unsafe((month + 1) % 12 + 1, is_leap)
                        },
                        lunar_day: unsafe { LunarDay::from_u8_unsafe(1) },
                    })
                } else {
                    let lunar_month = unsafe { LunarMonth::from_u8_unsafe(month, is_leap) };
                    Ok(LunisolarDate {
                        solar_year,
                        lunisolar_year,
                        lunar_month,
                        lunar_day: unsafe {
                            LunarDay::from_u8_unsafe(
                                lunar_month.get_total_days(lunisolar_year).unwrap()
                                    - day_diff as u8
                                    + 1,
                            )
                        },
                    })
                }
            } else {
                // 若天數差距沒比「西曆新年與對應農曆年新年」之天數差距小，表示此西曆日期已經進入下一個農曆年(已到達正月，從在農曆年頭開始)。
                day_diff -= new_year_diff; // 此時天數差距為西曆日期與對應農曆年第一天之距離(若西曆日期為2/23(此時天數差距為53)，而對應農曆年第一天是西曆的2/10(新年偏差為40)，則這兩個日期的天數差距為53-40=13)。

                let mut is_leap = false;

                let mut month = 1;

                loop {
                    let month_days = u16::from(
                        lunisolar_year
                            .get_total_days_in_a_month(unsafe {
                                LunarMonth::from_u8_unsafe(month, false)
                            })
                            .unwrap(),
                    );

                    if day_diff < month_days {
                        break;
                    }

                    day_diff -= month_days;

                    if month == leap_month {
                        if day_diff < leap_days {
                            is_leap = true;
                            break;
                        } else {
                            day_diff -= leap_days;
                        }
                    }

                    month += 1;
                }

                Ok(LunisolarDate {
                    solar_year,
                    lunisolar_year,
                    lunar_month: unsafe { LunarMonth::from_u8_unsafe(month, is_leap) },
                    lunar_day: unsafe { LunarDay::from_u8_unsafe(day_diff as u8 + 1) },
                })
            }
        }
    }

    /// 將西曆年月日轉成農曆年月日(包含西曆年)。
    pub fn from_solar_date(solar_date: SolarDate) -> Result<LunisolarDate, LunisolarError> {
        Self::from_solar_date_inner(solar_date, solar_date.to_naive_date())
    }

    /// 轉成西曆年月日。
    pub fn to_solar_date(self) -> SolarDate {
        SolarDate::from_lunisolar_date(self)
    }

    /// 將無時區的 `Chrono` 年月日實體，轉成農曆年月日(包含西曆年)。
    pub fn from_naive_date(naive_date: NaiveDate) -> Result<LunisolarDate, LunisolarError> {
        if naive_date < *MIN_LUNAR_DATE_IN_SOLAR_CALENDAR
            || naive_date > *MAX_LUNAR_DATE_IN_SOLAR_CALENDAR
        {
            Err(LunisolarError::OutOfLunarRange)
        } else {
            let solar_date = SolarDate::from_naive_date(naive_date)?;

            Self::from_solar_date_inner(solar_date, naive_date)
        }
    }

    /// 將有時區的 `Chrono` 年月日實體，依UTC時區轉成農曆年月日(包含西曆年)。
    pub fn from_date<Tz: TimeZone>(date: Date<Tz>) -> Result<LunisolarDate, LunisolarError> {
        let naive_date = date.naive_utc();

        Self::from_naive_date(naive_date)
    }

    /// 利用農曆西曆年和農曆月日來產生 `LunisolarDate` 實體。
    pub unsafe fn from_lunisolar_year_lunar_month_day_unsafe(
        lunisolar_year: LunisolarYear,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
    ) -> LunisolarDate {
        let n = Self::the_n_day_in_this_year_inner(lunisolar_year, lunar_month, lunar_day);

        let solar_year = lunisolar_year.to_solar_year();

        let year = solar_year.to_u16();

        let solar_year =
            if n + u16::from(NEW_YEAR_DIFFERENCE[(year - MIN_YEAR_IN_SOLAR_CALENDAR) as usize])
                >= solar_year.get_total_days()
            {
                SolarYear::from_u16(year + 1)
            } else {
                solar_year
            };

        LunisolarDate {
            solar_year,
            lunisolar_year,
            lunar_month,
            lunar_day,
        }
    }

    /// 利用農曆西曆年和農曆月日來產生 `LunisolarDate` 實體。
    pub fn from_lunisolar_year_lunar_month_day(
        lunisolar_year: LunisolarYear,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
    ) -> Result<LunisolarDate, LunisolarError> {
        if lunar_month.is_leap_month() {
            let leap_lunar_month = lunisolar_year.get_leap_lunar_month();

            match leap_lunar_month {
                Some(leap_lunar_month) => {
                    if lunar_month != leap_lunar_month {
                        return Err(LunisolarError::IncorrectLunarMonth);
                    }
                }
                None => return Err(LunisolarError::IncorrectLunarMonth),
            }
        }

        let days = lunar_month.get_total_days(lunisolar_year).unwrap();

        if lunar_day.to_u8() > days {
            Err(LunisolarError::IncorrectLunarDay)
        } else {
            Ok(unsafe {
                Self::from_lunisolar_year_lunar_month_day_unsafe(
                    lunisolar_year,
                    lunar_month,
                    lunar_day,
                )
            })
        }
    }

    /// 利用農曆西曆年和農曆月日來產生 `LunisolarDate` 實體。
    pub fn from_ymd(
        year: u16,
        month: u8,
        leap: bool,
        day: u8,
    ) -> Result<LunisolarDate, LunisolarError> {
        let lunisolar_year = match LunisolarYear::from_solar_year(year) {
            Some(lunisolar_year) => lunisolar_year,
            None => return Err(LunisolarError::IncorrectLunisolarYear),
        };

        let lunar_month = match LunarMonth::from_u8(month, leap) {
            Some(lunar_month) => lunar_month,
            None => return Err(LunisolarError::IncorrectLunarMonth),
        };

        let lunar_day = match LunarDay::from_u8(day) {
            Some(lunar_day) => lunar_day,
            None => return Err(LunisolarError::IncorrectLunarDay),
        };

        Self::from_lunisolar_year_lunar_month_day(lunisolar_year, lunar_month, lunar_day)
    }

    /// 以目前的年月日來產生 `LunisolarDate` 實體。
    pub fn now() -> Result<LunisolarDate, LunisolarError> {
        Self::from_date(Utc::now().date())
    }

    /// 用中文農曆西曆年和農曆月日字串來產生 `SolarDate` 實體。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<LunisolarDate, LunisolarError> {
        let s = s.as_ref();

        let year_index = {
            match s.find("　") {
                Some(index) => index,
                None => {
                    match s.find("年") {
                        Some(index) => index,
                        None => return Err(LunisolarError::IncorrectLunisolarYear),
                    }
                }
            }
        };

        let year_str = s[..year_index].trim();

        let lunisolar_year = match SolarYear::from_str(year_str) {
            Some(solar_year) => {
                match LunisolarYear::from_solar_year(solar_year) {
                    Some(lunisolar_year) => lunisolar_year,
                    None => return Err(LunisolarError::IncorrectLunisolarYear),
                }
            }
            None => return Err(LunisolarError::IncorrectLunisolarYear),
        };

        let s = &s[year_index + 3..];

        let month_index = {
            match s.find("月") {
                Some(index) => index,
                None => {
                    match s.find("　") {
                        Some(index) => index,
                        None => return Err(LunisolarError::IncorrectLunarMonth),
                    }
                }
            }
        };

        let month_str = s[..month_index + 3].trim();

        let month_str = {
            match month_str.find("年") {
                Some(index) => &month_str[index + 3..].trim(),
                None => {
                    match month_str.find("　") {
                        Some(index) => &month_str[index + 3..].trim(),
                        None => month_str,
                    }
                }
            }
        };

        let lunar_month = match LunarMonth::from_str(month_str) {
            Some(lunar_month) => lunar_month,
            None => return Err(LunisolarError::IncorrectLunarMonth),
        };

        let mut day_str = s[month_index + 3..].trim();

        if day_str.ends_with("日") {
            day_str = &day_str[..day_str.len() - 3];
        }

        let lunar_day = match LunarDay::from_str(day_str) {
            Some(lunar_day) => lunar_day,
            None => return Err(LunisolarError::IncorrectLunarDay),
        };

        Self::from_lunisolar_year_lunar_month_day(lunisolar_year, lunar_month, lunar_day)
    }

    /// 取得 `LunisolarDate` 實體所代表的中文農曆西曆年和農曆月日字串。
    pub fn to_chinese_string(self, chinese_variant: ChineseVariant) -> String {
        let mut s = String::new();

        self.write_to_chinese_string(chinese_variant, &mut s);

        s
    }

    /// 取得 `LunisolarDate` 實體所代表的中文農曆西曆年和農曆月日字串。
    pub fn write_to_chinese_string(self, chinese_variant: ChineseVariant, s: &mut String) {
        s.reserve(48);

        let lunisolar_year = self.lunisolar_year;

        lunisolar_year.to_solar_year().write_to_chinese_string(s);
        s.push_str("　");
        s.push_str(lunisolar_year.to_lunar_year().to_str());
        s.push_str("、");
        s.push_str(self.lunisolar_year.get_zodiac().to_str(chinese_variant));
        s.push_str("年");
        s.push_str("　");
        s.push_str(self.lunar_month.to_str(chinese_variant));
        s.push_str("　");
        s.push_str(self.lunar_day.to_str());
    }

    /// 取得西曆年。
    pub fn get_solar_year(self) -> SolarYear {
        self.solar_year
    }

    /// 取得農曆西曆年。
    pub fn get_lunisolar_year(self) -> LunisolarYear {
        self.lunisolar_year
    }

    /// 取得農曆年。
    pub fn get_lunar_year(self) -> LunarYear {
        self.lunisolar_year.to_lunar_year()
    }

    /// 取得農曆月。
    pub fn get_lunar_month(self) -> LunarMonth {
        self.lunar_month
    }

    /// 取得農曆日。
    pub fn get_lunar_day(self) -> LunarDay {
        self.lunar_day
    }

    /// 計算此農曆年月日是該農曆年的第幾天。舉例：2013/正月/初五，就是第五天。
    pub(crate) fn the_n_day_in_this_year_inner(
        lunisolar_year: LunisolarYear,
        lunar_month: LunarMonth,
        lunar_day: LunarDay,
    ) -> u16 {
        let mut n = u16::from(lunar_day.to_u8());

        let lunisolar_year = lunisolar_year;

        let month = lunar_month.to_u8();

        if lunar_month.is_leap_month() {
            for i in 1..=month {
                let lunar_month = unsafe { LunarMonth::from_u8_unsafe(i, false) };
                n += u16::from(lunar_month.get_total_days(lunisolar_year).unwrap());
            }
        } else {
            for i in 1..month {
                let lunar_month = unsafe { LunarMonth::from_u8_unsafe(i, false) };
                n += u16::from(lunar_month.get_total_days(lunisolar_year).unwrap());
            }

            if let Some(leap_lunar_month) = lunisolar_year.get_leap_lunar_month() {
                if lunar_month > leap_lunar_month {
                    n += lunisolar_year.get_total_days_in_leap_month_inner(leap_lunar_month) as u16;
                }
            }
        }

        n
    }

    /// 計算此農曆年月日是該農曆年的第幾天。舉例：2013/正月/初五，就是第五天。
    pub fn the_n_day_in_this_year(self) -> u16 {
        Self::the_n_day_in_this_year_inner(self.lunisolar_year, self.lunar_month, self.lunar_day)
    }

    /// 搭配出生時間(地支)，來計算八字有幾兩重。
    ///
    /// * 子：２３～１
    /// * 丑：１～３
    /// * 寅：３～５
    /// * 卯：５～７
    /// * 辰：７～９
    /// * 巳：９～１１
    /// * 午：１１～１３
    /// * 未：１３～１５
    /// * 申：１５～１７
    /// * 酉：１７～１９
    /// * 戌：１９～２１
    /// * 亥：２１～２３
    pub fn get_ba_zi_weight(self, earthly_branch: EarthlyBranch) -> f64 {
        let sum = self.lunisolar_year.to_lunar_year().get_ba_zi_weight()
            + self.lunar_month.get_ba_zi_weight()
            + self.lunar_day.get_ba_zi_weight()
            + earthly_branch.get_ba_zi_weight();

        f64::from(sum) / 10.0
    }

    /// 搭配出生時間，來計算八字有幾兩重。
    pub fn get_ba_zi_weight_by_time<T: Timelike>(self, time: T) -> f64 {
        let earthly_branch = EarthlyBranch::from_time(time);

        self.get_ba_zi_weight(earthly_branch)
    }
}

impl Display for LunisolarDate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_chinese_string(ChineseVariant::Traditional))
    }
}

impl FromStr for LunisolarDate {
    type Err = LunisolarError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LunisolarDate::from_str(s)
    }
}
