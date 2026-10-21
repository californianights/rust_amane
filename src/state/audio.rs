use amane::{Audio, Size, Text, Widget};

use crate::widgets::bar::label::Label;

pub fn label(audio: &Audio) -> Label {
    let volume = audio.volume();
    let glyph = if audio.muted() || volume == 0 {
        "󰖁"
    } else if volume < 34 {
        "󰕿"
    } else if volume < 67 {
        "󰖀"
    } else {
        "󰕾"
    };

    let text = format!("{glyph} {volume}%");
    let text_widget = Text::new(&text).size(15.0).weight(600);
    let Size::Fixed(text_width) = text_widget.width() else {
        unreachable!("audio label has fixed width");
    };

    Label {
        text,
        width: text_width + 24.0,
    }
}
