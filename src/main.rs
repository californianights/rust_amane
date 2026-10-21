mod state;
mod widgets;

fn main() {
    amane::App::new()
        .scale_factor(0.9)
        .font("JetBrainsMono Nerd Font")
        .ipc("toggle-launcher", widgets::launcher::toggle)
        .window(widgets::bar::view)
        .window(widgets::launcher::view)
        .run();
}
