use amane::Service;
use std::time::Duration;

pub struct Date {
    pub day: String,
}

impl Date {
    fn read_date() -> Self {
        let date_raw = amane::output("date '+%a %b %d'");
        let parts: Vec<&str> = date_raw.split_whitespace().take(3).collect();

        Self {
            day: parts.get(0).unwrap_or(&"").to_string(),
        }
    }
}

impl Service for Date {
    fn new() -> Self {
        Self::read_date()
    }

    fn interval() -> Duration {
        Duration::from_secs(60)
    }

    fn update(&mut self) -> bool {
        *self = Self::read_date();
        true
    }
}
