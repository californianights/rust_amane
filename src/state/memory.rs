use crate::widgets::bar::label::Label;
use amane::{Memory, Service};
use std::time::Duration;

pub struct MemoryService {
    pub label: Label,
    pub percent: u8,
}

impl MemoryService {
    fn read_memory() -> (Label, u8) {
        let memory = Memory::read();
        let percent = memory.percent();

        let text = format!(" {}%", percent);
        let width = if percent < 10 { 64.0 } else { 72.0 };

        (Label { text, width }, percent)
    }
}

impl Service for MemoryService {
    fn new() -> Self {
        let (label, percent) = Self::read_memory();
        Self { label, percent }
    }

    fn update(&mut self) -> bool {
        let (label, percent) = Self::read_memory();
        let changed = label.text != self.label.text || label.width != self.label.width;
        self.label = label;
        self.percent = percent;
        changed
    }

    fn interval() -> Duration {
        Duration::from_secs(5)
    }
}
