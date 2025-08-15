# Framework System TUI

![Rust](https://img.shields.io/badge/lang-rust-orange)  
![ratatui](https://img.shields.io/badge/ui-ratatui-blue)  
![Framework Laptop](https://img.shields.io/badge/hardware-Framework--Laptop-success)  

A terminal dashboard for controlling and monitoring your Framework Laptop hardware—charging, privacy, lighting, USB PD, and more—via direct API calls.

## Features

- [ ] **Live battery status** with limits and charging controls
- [ ] **Privacy toggles** for microphone & camera
- [ ] **Input Deck Mode** selector (Framework 16)
- [ ] **Keyboard & Fingerprint brightness** controls
- [ ] **USB PD port monitoring & reset**
- [ ] **Fan RPM and target setting**
- [ ] **Thermal info:** CPU/GPU temperatures
- [ ] **System info:** BIOS details
- [ ] **Log pane** for executed actions and errors
- [ ] **Accessible:** All controls visible at once, rapid navigation via keyboard

## Installation

Requires [Rust](https://rustup.rs/) (Edition 2024) and a Framework laptop.

```sh
git clone https://github.com/grouzen/framework-tool-tui.git
cd framework-tool-tui
cargo build --release
```

## Run

It needs to be running with root privileges to access hardware controls.

```sh
cargo build --release
sudo ./target/release/framework-tool-tui
```

## Architecture

See [`docs/mvp-plan.md`](docs/mvp-plan.md:1) for full technical breakdown.  
- Modular panel rendering and state management ([`src/app.rs`](src/app.rs:1), [`src/framework.rs`](src/framework.rs:1))
- Hardware polling via [`framework_lib`](https://github.com/FrameworkComputer/framework-system)
- Keyboard shortcuts and instant updates

## Contributing

Pull requests welcome! For bug reports or feature requests, see Issues.

## License

See [Cargo.toml](Cargo.toml:1) for license & dependencies.
