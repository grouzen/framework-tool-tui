# Framework System TUI

![Rust](https://img.shields.io/badge/lang-rust-orange) ![ratatui](https://img.shields.io/badge/ui-ratatui-blue) ![Framework Laptop](https://img.shields.io/badge/hardware-Framework--Laptop-success)  

A snappy TUI dashboard for controlling and monitoring your Framework Laptop hardware — charging, privacy, lighting, USB PD ports, and more.

![demo](/docs/screenshots/demo-v0.7.2.gif)

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

#### Arch linux

It is available via [AUR](https://aur.archlinux.org/packages/framework-tool-tui) or [archlinuxcn](https://github.com/archlinuxcn/repo/tree/master/archlinuxcn/framework-tool-tui)

```sh
[yay/paru] -S framework-tool-tui # Install from AUR
sudo pacman -S framework-tool-tui # Install from archlinuxcn
```

#### :beer: Homebrew for Linux

Available via the [`framework-tool-tui` formula](https://formulae.brew.sh/formula/framework-tool-tui)

```sh
brew install framework-tool-tui
```

### 😈 FreeBSD


It is available in [FreeBSD ports](https://www.freshports.org/sysutils/framework-tool-tui/)
```sh
pkg install framework-tool-tui
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

### Commit Message Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for automated versioning and changelog generation. Please format your commit messages as:

```
<type>: <description>

[optional body]
```

**Common types:**
- `feat:` - New feature (triggers minor version bump)
- `fix:` - Bug fix (triggers patch version bump)
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes

**Examples:**
```
feat: add thermal monitoring support
fix: correct battery percentage calculation
docs: update installation instructions
```

For breaking changes, add `!` after the type or include `BREAKING CHANGE:` in the footer:
```
feat!: redesign UI layout
```

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
