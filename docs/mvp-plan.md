# MVP Implementation Plan – Framework Laptop TUI (`ratatui` + `framework_lib`)

## Overall Architecture
- Structure as a Rust binary crate (`framework-tool-tui`) importing:
  - [`ratatui`](https://docs.rs/ratatui) for terminal UI
  - [`framework_lib`](https://github.com/FrameworkComputer/framework-system) for Framework Laptop hardware control
- Modular design: separate UI layout/render logic, application state model, and hardware control integration.

## Key Components

### 1. Module Structure
- `main.rs`: App entrypoint, sets up terminal, event loop, and root layout.
- `ui/`: Panel rendering functions for each of the 3×3 grid sections from `docs/tui-layout.md`.
- `state.rs`: Central `AppState` struct holding live values and pending changes for all controls.
- `framework_integration.rs`: Wrapper around `framework_lib` to query and update Mic/Cam, Input Deck mode, charging parameters, fingerprint, and keyboard brightness.

### 2. State Model
- Track current hardware state from `framework_lib` and a separate pending state for user edits.
- Each control panel reads from state and writes to pending state.
- Apply changes with `[Enter]`, reset with `[Esc]`.

### 3. Event Handling
- Global keybindings:
  - `[Tab]` → switch focus
  - `[Enter]` → apply focused control’s pending change
  - `[Esc]` → cancel pending change
  - `[Q]` → quit
- Event loop polls for keyboard input and refreshes from hardware state at intervals.

### 4. UI Rendering
- Use `ratatui::layout::Layout` to divide terminal into three rows, each split into three panels.
- Each panel = `Block` with title, current value, and interactive widget (toggle, slider, colour picker).
- Footer bar is static hints from `docs/tui-layout.md`.

### 5. `framework_lib` Integration (Replacing CLI Calls)
- Use direct APIs:
  - **Privacy**: `Mic::set_enabled()`, `Cam::set_enabled()`, and getters
  - **Input Deck Mode**: `InputDeck::current_mode()` / `.set_mode()`
  - **Charging**: `Battery::set_charge_limit()`, `.set_current_limit()`, `.set_rate_limit()` and corresponding getters
  - **Fingerprint Brightness**: `Fingerprint::set_brightness()`
  - **Keyboard Brightness**: `Keyboard::set_brightness()`
- Map each pending UI change to correct API call in `framework_integration.rs`.

### 6. Initialization
- On startup, query `framework_lib` to populate `AppState` with current values for all controls before first draw.

### 7. Error Handling
- Display API errors in log area.
- Disable unsupported controls (e.g., Input Deck on non-Framework 16) with notice.

## Milestones
1. Bootstrap terminal app with `ratatui` base layout and static text matching `docs/tui-layout.md`.
2. Implement `AppState` and periodic hardware polling using `framework_lib`.
3. Implement focus navigation and keyboard events.
4. Hook up Privacy controls to `framework_lib`.
5. Implement Charging controls.
6. Add Input Deck Mode control (detect Framework 16).
7. Add Fingerprint and Keyboard brightness controls.
8. Implement Feedback/Log panel.
9. Final testing and polishing.