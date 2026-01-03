use chrono::{DateTime, Datelike, Local, Timelike};

// Format time as HH:MM with leading zeros for the UI.
pub fn format_time(now: DateTime<Local>) -> String {
    format!("{:02}:{:02}", now.hour(), now.minute())
}

// Format date as "Вт, 12 мар" for the UI.
pub fn format_date(now: DateTime<Local>) -> String {
    format!(
        "{}, {:02} {}",
        weekday_ru(now.weekday().number_from_monday()),
        now.day(),
        month_ru(now.month())
    )
}

// Short Russian weekday names aligned with ISO weekday numbers.
fn weekday_ru(idx: u32) -> &'static str {
    match idx {
        1 => "Пн",
        2 => "Вт",
        3 => "Ср",
        4 => "Чт",
        5 => "Пт",
        6 => "Сб",
        _ => "Вс",
    }
}

// Short Russian month names aligned with chrono month numbers.
fn month_ru(idx: u32) -> &'static str {
    match idx {
        1 => "янв",
        2 => "фев",
        3 => "мар",
        4 => "апр",
        5 => "мая",
        6 => "июн",
        7 => "июл",
        8 => "авг",
        9 => "сен",
        10 => "окт",
        11 => "ноя",
        _ => "дек",
    }
}
