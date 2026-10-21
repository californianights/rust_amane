use amane::{
    Align, Center, Color, Full, Padding, Parent, Rectangle, Row, Size, Text, Widget, Workspace,
    Workspaces,
};

const WORKSPACE_WIDTH: f32 = 10.0;
const WORKSPACE_GAP: f32 = 3.0;
const OUTER_LABEL_PADDING: f32 = 18.0;
const WORKSPACE_LABEL_PADDING: f32 = 7.0;

fn label_text(label: &str) -> Text {
    Text::new(label).color(Color::WHITE).size(15.0).weight(600)
}

fn text_width(text: &Text) -> f32 {
    let Size::Fixed(width) = text.width() else {
        unreachable!("bar labels have fixed width");
    };

    width
}

fn label_padding(align: Align) -> (f32, f32) {
    match align {
        Align::Start => (OUTER_LABEL_PADDING, WORKSPACE_LABEL_PADDING),
        Align::End => (WORKSPACE_LABEL_PADDING, OUTER_LABEL_PADDING),
        Align::Center => (OUTER_LABEL_PADDING, OUTER_LABEL_PADDING),
    }
}

pub fn label_width(label: &str, align: impl Into<Align>) -> f32 {
    let (left_padding, right_padding) = label_padding(align.into());

    text_width(&label_text(label)) + left_padding + right_padding
}

pub fn view(label: &str, align: impl Into<Align>) -> Rectangle {
    let align = align.into();
    let (left_padding, right_padding) = label_padding(align);
    let text = label_text(label);

    Rectangle::new()
        .width(text_width(&text) + left_padding + right_padding)
        .height(Parent)
        .padding(Padding {
            top: 0.0,
            bottom: 0.0,
            left: left_padding,
            right: right_padding,
        })
        .align_child(align, Center)
        .child(text)
}

pub fn workspaces_width(count: usize) -> f32 {
    if count == 0 {
        return 0.0;
    }

    count as f32 * WORKSPACE_WIDTH + (count - 1) as f32 * WORKSPACE_GAP
}

pub fn workspaces(workspaces: &[Workspace]) -> Rectangle {
    let mut markers: Vec<Box<dyn Widget>> = Vec::new();

    for workspace in workspaces {
        let id = workspace.id();
        let (height, color) = if workspace.focused() {
            (14.0, Color::from("#A9BAD3"))
        } else {
            (6.0, Color::from("#A5BFE888"))
        };

        markers.push(Box::new(
            Rectangle::new()
                .width(WORKSPACE_WIDTH)
                .height(Parent)
                .on_click(move |_| Workspaces::focus(id))
                .align_child(Center, Center)
                .child(
                    Rectangle::new()
                        .width(3.0)
                        .height(height)
                        .fill(color)
                        .radius(Full),
                ),
        ));
    }

    Rectangle::new()
        .width(workspaces_width(workspaces.len()))
        .height(Parent)
        .child(Row::new(markers).gap(WORKSPACE_GAP))
}
