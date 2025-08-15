# Framework System TUI Architecture

This document explains the architecture and data flow for the Framework System TUI as of August 2025.

## Overview

The TUI dashboard manages hardware controls and state for Framework laptops. It uses Rust's `ratatui` library for UI, and interfaces with hardware through the `framework_lib` crate.

Core components:
- **App (src/app.rs)**: Handles the user interface and the main event/render loop.
- **Framework (src/framework.rs)**: Polls hardware, exposing controls and telemetry via `FrameworkControls`.
- **Main Entry (src/main.rs)**: Starts the app, sets up the TUI terminal environment.

## Component Diagram

```mermaid
graph LR
    Main["main.rs: main()"]
    App["app.rs: App"]
    Framework["framework.rs: Framework"]
    Controls["framework.rs: FrameworkControls"]
    Hardware["External: framework_lib (EC, power, SMBIOS, etc.)"]

    Main --> App
    App --> Framework
    Framework --> Controls
    Controls -->|read| App
    Framework -->|polls| Hardware
```

## Sequence Diagram: Data and Event Flow

```mermaid
sequenceDiagram
    participant User
    participant Terminal
    participant App
    participant Framework
    participant Hardware
    User->>Terminal: Keyboard Input
    Terminal->>App: Key Events
    App->>Framework: poll_if_needed()
    Framework->>Hardware: Read hardware APIs
    Hardware-->>Framework: Battery/Privacy/SMBIOS/Lighting
    Framework-->>App: Controls/Telemetry (FrameworkControls)
    App-->>Terminal: Render UI Panels
```

## Data Flow Diagram

```mermaid
flowchart TD
    Hardware["Hardware API (framework_lib)"] --> Framework["Framework Poll"]
    Framework --> Controls["FrameworkControls (State/Telemetry)"]
    Controls --> App["App (TUI Panels)"]
    App --> User["User (TUI Viewer)"]
    User --> App["Key events / interaction"]
```

## Component Descriptions

### App
- UI panels for battery, privacy, lighting, SMBIOS and more.
- Handles event loop: draws UI, reads keyboard actions, controls running state.
- Receives state from Framework (`self.framework.controls`).
- Forwards most hardware data queries to FrameworkControls.

### Framework
- Owns a `CrosEc` hardware control object.
- Periodically polls hardware (interval: `poll_interval`).
- Updates `FrameworkControls` struct, which extracts key data (battery stats, privacy, brightness, SMBIOS).
- Is isolated from TUI logic.

### FrameworkControls
- Holds the latest snapshot of all hardware data for UI rendering:
    - Battery: charge %, voltage, capacity, loss.
    - Privacy: mic/camera toggles.
    - Lighting: brightness levels.
    - SMBIOS: vendor, version, release date.

### Main
- Initializes terminal UI environment.
- Runs main loop (`App::run()`).
- Handles entering/exiting alternate screen and raw mode.

## Summary

The Framework System TUI is designed for clear separation between UI logic (App) and hardware access (Framework). Real-time data is regularly polled from hardware, then presented in multiple interactive panels.

License: See [Cargo.toml](Cargo.toml:1).