# Framework Laptop TUI Layout – Dashboard Grid (Option 2)

This document defines the **terminal-based user interface** layout for the Framework Laptop hardware control tool using `ratatui`.

We chose the **Dashboard Grid** layout because it enables **all controls and statuses to be visible at once**, minimizing navigation while giving quick access to both monitoring and control features.

---

## ASCII Layout

```
+================================================================================+
|                               FRAMEWORK SYSTEM TUI                             |
+================================================================================+
| BATTERY & CHARGING      | FAN CONTROL                  | USB PD PORTS         |
|-------------------------+------------------------------+----------------------|
| Charge %:  87% (Charging)  [Set Limit: 80%]             | Port1: Enabled (PD)  |
| Limit:     80%              [Set Current: 65W]          | Port2: Disabled      |
| CurrLim:   65W              [Set Rate: 45W/h]           | [Enable/Disable]     |
| RateLim:   45W/h                                         | [Reset Ports]        |
+================================================================================+
| PRIVACY CONTROLS        | INPUT DECK MODE              | KEYBOARD LIGHTING    |
|-------------------------+------------------------------+----------------------|
| Mic: Enabled      [Toggle] | Mode: Gaming [Change]      | Brightness: 50% [Adj]|
| Cam: Disabled     [Toggle] |                           | RGB: #FF0000 [Pick]  |
+================================================================================+
| FINGERPRINT BRIGHTNESS  | SYSTEM INFO                  | MESSAGES / LOG       |
|-------------------------+------------------------------+----------------------|
| Brightness: 30% [Adj]   | CPU Temp:  55°C              | Last: OK             |
|                         | GPU Temp:  48°C              | Fan RPM changed      |
|                         | Model: Framework 16          | PD Port1 Enabled     |
+================================================================================+
| [Tab] Switch Focus  [Enter] Apply  [Esc] Cancel  [Q] Quit                      |
+================================================================================+
```

---

## Summary of Structure

- **Unified Rows:** Each control panel row presents the live **current value** followed immediately by its control (toggle, slider, input, etc.) — no duplication between “show” and “set” areas.
- **Top Bar:** Application title.
- **Panels:** Arranged in a responsive 3×3 grid:
  - **Battery & Charging:** Displays status and controls for charge limit, current limit, and rate limit.
  - **Fan Control:** Shows RPM/duty and lets the user change target speed or toggle auto control.
  - **USB PD Ports:** Displays each port and state, with control buttons for enable/disable/reset.
  - **Privacy Controls:** Mic and Cam status plus quick toggles.
  - **Input Deck Mode:** Shows current mode with selection control (Framework 16 only).
  - **Keyboard Lighting:** Brightness and RGB controls.
  - **Fingerprint Brightness:** Brightness control for fingerprint reader LED.
  - **System Info:** CPU/GPU temps, model details.
  - **Messages/Log:** Feedback area for executed commands.
- **Footer Bar:** Static keybinding hints.

---

## Implementation Notes in `ratatui`

- Layout can be implemented with `ratatui::layout::Layout` divided into vertical chunks, each split horizontally for panels.
- Each panel can be a `Block` containing:
  - **Status**: Rendered as `Paragraph` or `Spans` for read-only text.
  - **Control**: Rendered as interactive widget (slider, list, toggle).
- Each interactive element changes **pending state**, applied when `[Enter]` is pressed.

---

This layout ensures **fast hardware adjustments** while keeping **real-time system state** visible at all times.