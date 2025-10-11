# Framework System TUI

![Rust](https://img.shields.io/badge/lang-rust-orange) ![ratatui](https://img.shields.io/badge/ui-ratatui-blue) ![Framework Laptop](https://img.shields.io/badge/hardware-Framework--Laptop-success)  

A terminal dashboard for controlling and monitoring your Framework Laptop hardware — charging, privacy, lighting, USB PD, and more — via direct API calls.

## Features

- [x] **Live battery status** with limits and charging controls
- [x] **Privacy toggles** for microphone & camera
- [ ] **Input Deck Mode** selector (Framework 16)
- [x] **Keyboard & Fingerprint brightness** controls
- [x] **USB PD port monitoring & reset**
- [x] **Fan RPM and target setting**
- [ ] **Thermal info:** CPU/GPU temperatures
- [x] **System info:** BIOS details
- [ ] **Log pane** for executed actions and errors
- [x] **Accessible:** All controls visible at once, rapid navigation via keyboard

## Screenshots

Dashboard

![dashboard](/docs/screenshots/dashboard-mode.png)

Switching between panels and setting values

![adjust-value](/docs/screenshots/adjust-value.png)

## Installation

### :dvd: Binaries

The pre-compiled binaries for Linux are available for download on the [Releases](https://github.com/grouzen/framework-tool-tui/releases) page

### :memo: From source

Requires [Rust](https://rustup.rs/) (Edition 2024) and a Framework laptop.

```sh
git clone https://github.com/grouzen/framework-tool-tui.git
cd framework-tool-tui
cargo build --release
# or via cargo install
cargo install --path .
```

### :penguin: Linux distros

#### Gentoo linux

It is available via `lamdness` overlay

```sh
sudo eselect repository enable lamdness
sudo emaint -r lamdness sync
sudo emerge -av app-laptop/framework-tool-tui
```


## Run

It needs to be running with root privileges to access hardware controls.

```sh
sudo framework-tool-tui
```

## Architecture

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for full technical breakdown.  
- Modular panel rendering and state management ([`src/app.rs`](src/app.rs:1), [`src/framework.rs`](src/framework.rs:1))
- Hardware polling via [`framework_lib`](https://github.com/FrameworkComputer/framework-system)
- Keyboard shortcuts and instant updates

## Contributing

Pull requests welcome! For bug reports or feature requests, see Issues.

## License

See [LICENSE](LICENSE) and [Cargo.toml](Cargo.toml:1) for license & dependencies.
