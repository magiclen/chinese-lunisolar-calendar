use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum LunisolarError {
    /// 超出西曆支援的日期範圍。
    OutOfSolarRange,
    /// 錯誤的西曆年。
    IncorrectSolarYear,
    /// 錯誤的西曆月。
    IncorrectSolarMonth,
    /// 錯誤的西曆日。
    IncorrectSolarDay,
    /// 超出農曆支援的日期範圍。
    OutOfLunarRange,
    /// 錯誤的農曆西曆年。
    IncorrectLunisolarYear,
    /// 錯誤的農曆月。
    IncorrectLunarMonth,
    /// 錯誤的農曆日。
    IncorrectLunarDay,
}

impl Display for LunisolarError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            LunisolarError::OutOfSolarRange => f.write_str("The solar date range is out of range."),
            LunisolarError::IncorrectSolarYear => f.write_str("The solar year is incorrect."),
            LunisolarError::IncorrectSolarMonth => f.write_str("The solar month is incorrect."),
            LunisolarError::IncorrectSolarDay => f.write_str("The solar day is incorrect."),
            LunisolarError::OutOfLunarRange => f.write_str("The lunar date range is out of range."),
            LunisolarError::IncorrectLunisolarYear => f.write_str("The lunar year is incorrect."),
            LunisolarError::IncorrectLunarMonth => f.write_str("The lunar month is incorrect."),
            LunisolarError::IncorrectLunarDay => f.write_str("The lunar day is incorrect."),
        }
    }
}

impl Error for LunisolarError {}
