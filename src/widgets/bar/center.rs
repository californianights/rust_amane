use amane::{Align, Center, Color, Padding, Parent, Rectangle, Text};

pub fn view(label: &str, align: impl Into<Align>) -> Rectangle {
    Rectangle::new()
        .width(65.0)
        .height(Parent)
        .padding(Padding {
            top: 0.0,
            bottom: 0.0,
            left: 18.0,
            right: 18.0,
        })
        .align_child(align, Center)
        .child(Text::new(label).color(Color::WHITE).size(15.0).weight(600))
}
