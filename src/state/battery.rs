use crate::widgets::bar::label::Label;
use amane::{Battery, Service};
use std::time::Duration;

pub struct BatteryService {
    pub label: Label,
    pub percent: u8,
}

pub(crate) fn battery_glyph(percent: u8) -> &'static str {
    match percent {
        0..=9 => "󰁺",
        10..=19 => "󰁻",
        20..=29 => "󰁼",
        30..=39 => "󰁽",
        40..=49 => "󰁾",
        50..=59 => "󰁿",
        60..=69 => "󰂀",
        70..=79 => "󰂁",
        80..=89 => "󰂂",
        _ => "󰁹",
    }
}

impl BatteryService {
    fn read_battery() -> (Label, u8) {
        let battery = Battery::read();
        let percent = battery.percent();

        let text = format!("{} {}%", battery_glyph(percent), percent);
        let width = if percent == 100 {
            80.0
        } else if percent < 10 {
            64.0
        } else {
            72.0
        };

        (Label { text, width }, percent)
    }
}

impl Service for BatteryService {
    fn new() -> Self {
        let (label, percent) = Self::read_battery();
        Self { label, percent }
    }

    fn update(&mut self) -> bool {
        let (label, percent) = Self::read_battery();
        let changed = label.text != self.label.text || label.width != self.label.width;
        self.label = label;
        self.percent = percent;
        changed
    }

    fn interval() -> Duration {
        Duration::from_secs(60)
    }
}
