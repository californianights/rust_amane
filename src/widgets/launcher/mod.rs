use std::time::Duration;

use amane::{
    Align, Apps, Center, Color, Column, DesktopApp, Full, Key, Keyboard, LayerWindow, Padding,
    Parent, Rectangle, Service, Start, Text, TextInput, TextStyle, Weight, Zone, children,
};

const SEARCH_ID: &str = "launcher-search";
const MAX_RESULTS: usize = 7;

const MENU_WIDTH: f32 = 560.0;
const MENU_HEIGHT: f32 = 490.0;
const ROW_HEIGHT: f32 = 36.0;

struct Launcher {
    open: bool,
    query: String,
    selected: usize,
}

impl Service for Launcher {
    fn new() -> Self {
        Self {
            open: false,
            query: String::new(),
            selected: 0,
        }
    }

    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

fn filtered_apps(query: &str) -> Vec<DesktopApp> {
    if query.trim().is_empty() {
        return Vec::new();
    }

    let query = query.to_lowercase();

    Apps::read()
        .list()
        .iter()
        .filter(|app| app.name().to_lowercase().contains(&query))
        .take(MAX_RESULTS)
        .cloned()
        .collect()
}

fn close() {
    {
        let mut launcher = Launcher::write();
        launcher.open = false;
        launcher.query.clear();
        launcher.selected = 0;
    }

    TextInput::set_text(SEARCH_ID, "");
}

fn launch_selected() {
    let (query, selected) = {
        let launcher = Launcher::read();
        (launcher.query.clone(), launcher.selected)
    };

    let app = filtered_apps(&query).into_iter().nth(selected);

    if let Some(app) = app {
        app.launch();
        close();
    }
}

fn move_selection(backward: bool) {
    let query = Launcher::read().query.clone();
    let count = filtered_apps(&query).len();

    if count == 0 {
        return;
    }

    let mut launcher = Launcher::write();

    launcher.selected = if backward {
        if launcher.selected == 0 {
            count - 1
        } else {
            launcher.selected - 1
        }
    } else {
        (launcher.selected + 1) % count
    };
}

pub fn view() -> LayerWindow {
    let (open, query, selected) = {
        let launcher = Launcher::read();

        (launcher.open, launcher.query.clone(), launcher.selected)
    };

    let apps = filtered_apps(&query);

    let search = TextInput::new(SEARCH_ID)
        .width(Parent)
        .text_style(TextStyle::new().size(36.0).color(Color::from("#ffffff")))
        .text_align(Center)
        .placeholder(
            Text::new("Search...")
                .size(40.0)
                .color(Color::from("#a5a5a5"))
                .weight(Weight::Light)
                .align(Center),
        )
        .focused()
        .on_change(|query| {
            let mut launcher = Launcher::write();
            launcher.query = query;
            launcher.selected = 0;
        })
        .on_submit(|_| launch_selected());

    let mut rows: Vec<Box<dyn amane::Widget>> = Vec::new();

    for (index, app) in apps.iter().enumerate() {
        let name = app.name().to_string();
        let row = Rectangle::new()
            .width(Parent)
            .height(ROW_HEIGHT)
            .radius(6.0)
            .padding(Padding {
                top: 0.0,
                right: 12.0,
                bottom: 0.0,
                left: 12.0,
            })
            .align_child(Align::Center, Align::Center)
            .child(
                amane::Text::new(name)
                    .size(18.0)
                    .color(Color::from("#c5bfd2")),
            )
            .on_click(move |_| {
                let app = filtered_apps(&Launcher::read().query)
                    .into_iter()
                    .nth(index);

                if let Some(app) = app {
                    app.launch();
                    close();
                }
            });

        rows.push(Box::new(row));
    }

    let menu = Rectangle::new()
        .width(MENU_WIDTH)
        .height(MENU_HEIGHT)
        .radius(12.0)
        .padding(Padding {
            top: 100.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        })
        .align_child(Center, Center)
        .child(
            Column::new(children![
                Rectangle::new()
                    .width(Parent)
                    .height(48.0)
                    .align_child(Align::End, Center)
                    .child(search),
                Column::new(rows).width(Parent).height(Parent).gap(8.0),
            ])
            .width(Parent)
            .height(Parent)
            .align(Center)
            .gap(8.0),
        );

    let backdrop = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill("#000000b8")
        .align_child(Align::Center, Align::Start)
        .child(menu)
        .on_click(|_| close());

    LayerWindow::new()
        .width(Full)
        .height(Full)
        .visible(open)
        .keyboard(Keyboard::Exclusive)
        .space(Zone::Ignore)
        .on_key(|key| match key {
            Key::Escape => close(),
            Key::Tab => move_selection(false),
            Key::Up => move_selection(true),
            Key::Down => move_selection(false),
            _ => {}
        })
        .child(backdrop)
}

pub fn toggle(_: &[String]) -> String {
    let opened = {
        let mut launcher = Launcher::write();

        launcher.open = !launcher.open;
        launcher.query.clear();
        launcher.selected = 0;

        launcher.open
    };

    TextInput::set_text(SEARCH_ID, "");

    if opened {
        String::from("opened")
    } else {
        String::from("closed")
    }
}
