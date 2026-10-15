mod state;
mod widgets;

fn main() {
    amane::App::new()
        .scale_factor(0.9)
        .font("JetBrainsMono Nerd Font")
        .window(widgets::bar::view)
        .run();
}
