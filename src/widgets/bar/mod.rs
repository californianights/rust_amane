pub mod center;
pub mod label;
pub mod left;
pub mod right;

use crate::state::audio;
use crate::state::battery::BatteryService;
use crate::state::bluetooth;
use crate::state::clock::Clock;
use crate::state::cpu::CpuService;
use crate::state::date::Date;
use crate::state::memory::MemoryService;
use crate::state::network::NetworkService;
use amane::{
    Center, End, Full, Layer, LayerWindow, Margin, Parent, Rectangle, Row, Service, Stack, Start,
    Vertical, Workspaces, Zone, children,
};

pub fn view() -> LayerWindow {
    let clock = Clock::read();
    let workspaces = Workspaces::read();
    let date = Date::read();
    let network = NetworkService::read();
    let audio = amane::Audio::read();
    let audio_label = audio::label(&audio);
    let battery = BatteryService::read();
    let bluetooth = amane::Bluetooth::read();
    let bluetooth_label = bluetooth::label(&bluetooth);
    let cpu = CpuService::read();
    let memory = MemoryService::read();
    let workspace_list = workspaces.list();
    let workspace_width = center::workspaces_width(workspace_list.len());
    let center_gap = if workspace_list.is_empty() { 0.0 } else { 3.0 };
    let center_width = center::label_width(&date.day, Start)
        + center::label_width(&clock.time, End)
        + workspace_width
        + center_gap * 2.0;

    let mut center_widgets: Vec<Box<dyn amane::Widget>> =
        vec![Box::new(center::view(&date.day, Start))];

    if !workspace_list.is_empty() {
        center_widgets.push(Box::new(center::workspaces(workspace_list)));
    }

    center_widgets.push(Box::new(center::view(&clock.time, End)));

    LayerWindow::new()
        .width(Full)
        .height(32.0)
        .margin(Margin {
            top: 8,
            right: 0,
            bottom: 0,
            left: 8,
        })
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Bottom)
        .space(Zone::Reserve)
        .child(Stack::new(children![
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .child(Row::new(children![
                    left::view(&[&cpu.label, &memory.label]),
                    right::view(&[
                        &bluetooth_label,
                        &audio_label,
                        &battery.label,
                        &network.label,
                    ]),
                ])),
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .align_child(Center, Center)
                .child(
                    Rectangle::new()
                        .width(center_width)
                        .height(Parent)
                        .fill("#A5BFE833")
                        .radius(Full)
                        .child(Row::new(center_widgets).gap(center_gap)),
                )
        ]))
}
