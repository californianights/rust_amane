use amane::{Center, Color, End, Full, Gradient, Parent, Rectangle, Row, Text, Widget};

use crate::widgets::bar::label::Label;

pub fn view(labels: &[&Label]) -> Rectangle {
    let gap = 8.0;
    let padding_x = 8.0;

    let mut children: Vec<Box<dyn Widget>> = Vec::new();
    let mut total_width = 2.0 * padding_x;

    for (i, label) in labels.iter().enumerate() {
        if i > 0 {
            total_width += gap;
            children.push(Box::new(Rectangle::new().width(gap).height(Parent)));
        }

        total_width += label.width;

        children.push(Box::new(
            Rectangle::new()
                .width(label.width)
                .height(Parent)
                .fill("#A5BFE833")
                .radius(Full)
                .align_child(Center, Center)
                .child(
                    Text::new(&label.text)
                        .color(Color::WHITE)
                        .size(15.0)
                        .weight(600),
                ),
        ));
    }

    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(End, Center)
        .child(
            Rectangle::new()
                .width(total_width)
                .height(Parent)
                .align_child(Center, Center)
                .child(Row::new(children)),
        )
}
