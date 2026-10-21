use amane::{Bluetooth, BluetoothDevice, Size, Text, Widget};

use crate::widgets::bar::label::Label;

use super::battery::battery_glyph;

fn device_glyph(icon: &str) -> Option<&'static str> {
    match icon {
        "audio-headset" => Some("󰋎"),
        "audio-headphones" => Some("󰋋"),
        "input-gaming" => Some("󰊗"),
        _ => None,
    }
}

pub fn label(bluetooth: &Bluetooth) -> Label {
    let connected: Vec<&BluetoothDevice> = bluetooth
        .devices()
        .iter()
        .filter(|device| device.connected())
        .collect();

    let mut parts = Vec::new();

    if connected.is_empty() {
        parts.push(if bluetooth.powered() {
            "󰂯 ON".to_string()
        } else {
            "󰂲 OFF".to_string()
        });
    } else {
        for device in connected {
            if let Some(glyph) = device_glyph(device.icon()) {
                parts.push(glyph.to_string());
            }

            if let Some(percent) = device.battery() {
                parts.push(battery_glyph(percent).to_string());
            }
        }
    }

    let text = parts.join(" ");
    let label = Text::new(&text).size(15.0).weight(600);
    let Size::Fixed(text_width) = label.width() else {
        unreachable!("Bluetooth labels have fixed width");
    };
    let width = text_width + 24.0;

    Label { text, width }
}

#[cfg(test)]
mod tests {
    use super::device_glyph;

    #[test]
    fn maps_supported_bluetooth_device_icons() {
        assert_eq!(device_glyph("audio-headset"), Some("󰋎"));
        assert_eq!(device_glyph("audio-headphones"), Some("󰋋"));
        assert_eq!(device_glyph("input-gaming"), Some("󰊗"));
        assert_eq!(device_glyph("unknown"), None);
    }
}
