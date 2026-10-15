pub mod center;
pub mod label;
pub mod left;
pub mod right;

use crate::state::battery::BatteryService;
use crate::state::clock::Clock;
use crate::state::cpu::CpuService;
use crate::state::date::Date;
use crate::state::memory::MemoryService;
use crate::state::network::NetworkService;
use amane::{
    Center, End, Full, Gradient, Layer, LayerWindow, Margin, Parent, Rectangle, Row, Service,
    Stack, Start, Vertical, Workspaces, Zone, children,
};

pub fn view() -> LayerWindow {
    let clock = Clock::read();
    let workspaces = Workspaces::read();
    let date = Date::read();
    let network = NetworkService::read();
    let battery = BatteryService::read();
    let cpu = CpuService::read();
    let memory = MemoryService::read();

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
        .layer(Layer::Top)
        .space(Zone::Reserve)
        .child(Stack::new(children![
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .child(Row::new(children![
                    left::view(workspaces.list().to_vec()),
                    right::view(&[&cpu.label, &memory.label, &battery.label, &network.label,]),
                ])),
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .align_child(Center, Center)
                .child(
                    Rectangle::new()
                        .width(130.0)
                        .height(Parent)
                        .fill(Gradient::linear(
                            135.0,
                            [(0.0, "#0d1422"), (1.0, "#05080f")],
                        ))
                        .radius(Full)
                        .child(Row::new(children![
                            center::view(&date.day, Start),
                            center::view(&clock.time, End),
                        ])),
                )
        ]))
}
