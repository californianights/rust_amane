use crate::widgets::bar::label::Label;
use amane::{Cpu, Memory, Service};
use std::time::Duration;

pub struct CpuService {
    pub label: Label,
    pub percent: u8,
}

impl CpuService {
    fn read_cpu() -> (Label, u8) {
        let cpu = Cpu::read();
        let percent = cpu.percent();

        let text = format!(" {}%", percent);
        let width = if percent < 10 { 64.0 } else { 72.0 };

        (Label { text, width }, percent)
    }
}

impl Service for CpuService {
    fn new() -> Self {
        let (label, percent) = Self::read_cpu();
        Self { label, percent }
    }

    fn update(&mut self) -> bool {
        let (label, percent) = Self::read_cpu();
        let changed = label.text != self.label.text || label.width != self.label.width;
        self.label = label;
        self.percent = percent;
        changed
    }

    fn interval() -> Duration {
        Duration::from_secs(5)
    }
}
