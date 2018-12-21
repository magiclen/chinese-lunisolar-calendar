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