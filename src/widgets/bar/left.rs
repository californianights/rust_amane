use amane::{
    Center, Color, Full, Gradient, Padding, Parent, Rectangle, Row, Start, Text, Widget, Workspace,
    Workspaces,
};

pub fn view(workspaces: Vec<Workspace>) -> Rectangle {
    let button_w = 34.0;
    let gap = 4.0;
    let padding_x = 8.0;
    let count = workspaces.len() as f32;

    let width = if count == 0.0 {
        2.0 * padding_x
    } else {
        (count * button_w) + ((count - 1.0) * gap) + (2.0 * padding_x)
    };

    let mut children: Vec<Box<dyn Widget>> = Vec::new();

    for (i, ws) in workspaces.iter().enumerate() {
        if i > 0 {
            children.push(Box::new(Rectangle::new().width(gap).height(Parent)));
        }
        children.push(Box::new(workspace_button(ws)));
    }

    Rectangle::new()
        .width(width)
        .height(Parent)
        .fill(Gradient::linear(
            135.0,
            [(0.0, "#0d1422"), (1.0, "#05080f")],
        ))
        .radius(Full)
        .padding(Padding {
            top: 2.0,
            right: padding_x,
            bottom: 2.0,
            left: padding_x,
        })
        .align_child(Start, Center)
        .child(Row::new(children))
}

fn workspace_button(workspace: &Workspace) -> Rectangle {
    let id = workspace.id();
    let label = workspace.index().to_string();

    let button = Rectangle::new()
        .width(34.0)
        .height(28.0)
        .radius(Full)
        .align_child(Center, Center)
        .on_click(move |_| Workspaces::focus(id));

    if workspace.focused() {
        button
            .fill(Gradient::linear(
                135.0,
                [(0.0, "#c0caf5"), (1.0, "#334050")],
            ))
            .child(
                Text::new(label)
                    .color(Color::from("#040509"))
                    .size(15.0)
                    .weight(600),
            )
    } else {
        button.child(Text::new(label).color(Color::WHITE).size(15.0).weight(600))
    }
}
