use crate::widgets::bar::label::Label;
use amane::{Link, Service};
use std::time::Duration;

pub struct NetworkService {
    pub label: Label,
    pub link: Link,
    pub strength: u8,
}

impl NetworkService {
    fn read_network() -> (Label, u8, Link) {
        let network = amane::Network::read();
        let link = network.link();
        let strength = network.strength();

        let text = match link {
            Link::Offline => "󰤮  Disconnected".to_string(),
            Link::Wired => "󰈁 Wired".to_string(),
            Link::Wifi => {
                let glyph = match strength {
                    0 => "󰤯",
                    1..=25 => "󰤟",
                    26..=50 => "󰤢",
                    51..=75 => "󰤥",
                    _ => "󰤨",
                };
                format!("{}  {}", glyph, network.ssid())
            }
            Link::Other => "Connected".to_string(),
        };

        let width = text.len() as f32 * 9.0;
        (Label { text, width }, strength, link)
    }
}

impl Service for NetworkService {
    fn new() -> Self {
        let (label, strength, link) = Self::read_network();
        Self {
            label,
            strength,
            link,
        }
    }

    fn update(&mut self) -> bool {
        let (label, strength, link) = Self::read_network();
        let changed = label.text != self.label.text || label.width != self.label.width;
        self.label = label;
        self.strength = strength;
        self.link = link;
        changed
    }

    fn interval() -> Duration {
        Duration::from_secs(1)
    }
}
