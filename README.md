# Framework System TUI

![Rust](https://img.shields.io/badge/lang-rust-orange) ![ratatui](https://img.shields.io/badge/ui-ratatui-blue) ![Framework Laptop](https://img.shields.io/badge/hardware-Framework--Laptop-success)  

A snappy TUI dashboard for controlling and monitoring your Framework Laptop hardware — charging, privacy, lighting, USB PD ports, and more.

## :octocat: Features

### Framework Laptop Hardware

- [x] **Live battery and charge status** with limits and charging controls
- [x] **Privacy toggles** for microphone & camera
- [ ] **Input Deck Mode** selector (Framework 16)
- [x] **Keyboard & Fingerprint brightness** controls
- [x] **USB PD port monitoring**
- [x] **Fan RPM**
- [ ] **Thermal info:** CPU/GPU temperatures
- [x] **System info:** BIOS details

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


## :man: Run

It needs to be running with root privileges to access hardware controls.

```sh
sudo framework-tool-tui
```

## :pencil: Architecture

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for full technical breakdown.  
- Hardware polling via [`framework_lib`](https://github.com/FrameworkComputer/framework-system)

## :handshake: Contributing

Pull requests welcome! For bug reports or feature requests, see Issues.

### Technical stack

- [ratatui](https://ratatui.rs)
- [tokio](https://tokio.rs)
- [framework_lib](https://github.com/FrameworkComputer/framework-system/tree/main/framework_lib)

## :judge: License

```
MIT License

Copyright (c) 2025 Mykhailo Nedokushev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```

See [LICENSE](LICENSE) and [Cargo.toml](Cargo.toml:1) for license & dependencies.
