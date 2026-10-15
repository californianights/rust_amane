use amane::Service;
use std::time::Duration;

pub struct Clock {
    pub time: String,
}

impl Service for Clock {
    fn new() -> Self {
        Self {
            time: amane::output("date +%H:%M:%S"),
        }
    }

    fn interval() -> Duration {
        Duration::from_secs(1)
    }

    fn update(&mut self) -> bool {
        let time = amane::output("date +%H:%M");
        let changed = time != self.time;
        self.time = time;
        changed
    }
}
