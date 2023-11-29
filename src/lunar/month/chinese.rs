/// 列舉農曆十二個月份名稱：正月、二月、三月、四月、五月、六月、七月、八月、九月、十月、冬月、臘月。閏月緊接其後。
pub(super) const THE_LUNAR_MONTHS: [(&str, &str); 31] = [
    ("正月", "正月"),
    ("二月", "二月"),
    ("三月", "三月"),
    ("四月", "四月"),
    ("五月", "五月"),
    ("六月", "六月"),
    ("七月", "七月"),
    ("八月", "八月"),
    ("九月", "九月"),
    ("十月", "十月"),
    ("冬月", "冬月"),
    ("臘月", "腊月"),
    ("閏正月", "闰正月"),
    ("閏二月", "闰二月"),
    ("閏三月", "闰三月"),
    ("閏四月", "闰四月"),
    ("閏五月", "闰五月"),
    ("閏六月", "闰六月"),
    ("閏七月", "闰七月"),
    ("閏八月", "闰八月"),
    ("閏九月", "闰九月"),
    ("閏十月", "闰十月"),
    ("閏冬月", "闰冬月"),
    ("閏臘月", "闰腊月"),
    ("一月", "一月"),         // 補充
    ("十一月", "十一月"),     // 補充
    ("十二月", "十二月"),     // 補充
    ("闰一月", "閏一月"),     // 補充
    ("闰十一月", "閏十一月"), // 補充
    ("闰十二月", "閏十二月"), // 補充
    ("闰臘月", "閏腊月"),     // 排列組合補充
];