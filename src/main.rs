use amane::{
    Align, App, Center, Color, End, Full, LayerWindow, Parent, Rectangle, Row, Start, Text,
    Vertical, Zone, children,
};
fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    LayerWindow::new()
        .width(Full)
        .height(30.0)
        .anchor_vertical(Vertical::Top)
        .space(Zone::Reserve)
        .child(Row::new(children![
            section(Text::new("workspaces").color(Color::WHITE), Start),
            section(Text::new("12:00").color(Color::WHITE), Center),
            section(Text::new("battery").color(Color::WHITE), End),
        ]))
}

fn section(content: Text, align: impl Into<Align>) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill("#1e1e2e")
        .align_child(align, Center)
        .child(content)
}
