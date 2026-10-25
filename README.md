# rust-amane

A small desktop panel and application launcher written in Rust. This project uses **Amane**, a framework that I have modified slightly; it is not an unmodified upstream version.

## What it does

- **Panel:** shown at the top of the screen. CPU and memory usage appear on the left; the date, workspaces, and clock are centered; Bluetooth, audio volume, battery, and network status appear on the right.
- **Application launcher:** search installed applications, select a result with the keyboard, and launch it with Enter or a click. Escape closes the launcher; Tab and the arrow keys move the selection.
- **IPC:** the `toggle-launcher` command opens or closes the launcher.

## Project layout

```text
src/
├── main.rs             # application setup, windows, and IPC
├── state/              # services and state for system information
└── widgets/
    ├── bar/            # panel and its left, center, and right sections
    └── launcher/       # application search and launch behavior
```

Services in `state/` refresh their data at different intervals. Window composition and widget layout live in `widgets/`.

## Develop, build, and run

`Cargo.toml` is kept for Neovim's Rust LSP support; it is not the project's build or run entry point. The project uses the Amane CLI:

```sh
amane compile
```

Run the application with:

```sh
amane run
```

For live updates while developing features or fixes, use:

```sh
amane dev
```

The application scale (`0.9`) and font (`JetBrainsMono Nerd Font`) are configured in `src/main.rs`.
