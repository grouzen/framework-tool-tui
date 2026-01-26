# Plan: Add Thermal Panel with FAN Speed Graph

Add a new "Thermal" panel positioned to the left of the PD Ports panel, displaying first FAN's RPM with a historical graph (auto-scaled Y-axis with 10% padding). The FAN data (`fan_rpm`) is already available in `FrameworkInfo`.

---

## Step 1: Create Graph Component

**File:** `src/tui/component/thermal_graph_panel.rs`

Create a new component that displays a historical graph of FAN RPM using the `bgraph` crate.

### Structure

```rust
use bgraph::{ColorGradient, GradientMode, Graph, TimeSeriesState};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    framework::info::FrameworkInfo,
    tui::{component::Component, theme::Theme},
};

const HISTORY_SIZE: usize = 200;

pub struct ThermalGraphPanelComponent {
    fan_rpm_series: TimeSeriesState,
}
```

### Key Implementation Details

1. **Auto-scaling with 10% padding:**
   - Track the maximum value in the series
   - Compute `y_max = max_rpm * 1.1` (10% padding above max)
   - Use a minimum floor (e.g., 100.0) to avoid division issues when fan is off

2. **Constructor:**
   ```rust
   impl ThermalGraphPanelComponent {
       pub fn new() -> Self {
           Self {
               // Initial range doesn't matter much since we auto-scale
               fan_rpm_series: TimeSeriesState::with_range(HISTORY_SIZE, 0.0, 6000.0),
           }
       }
   }
   ```

3. **Update history from FrameworkInfo:**
   ```rust
   fn update_history(&mut self, info: &FrameworkInfo) {
       let rpm = info.fan_rpm
           .as_ref()
           .and_then(|rpms| rpms.first())
           .map(|&r| r as f32)
           .unwrap_or(0.0);
       self.fan_rpm_series.push(rpm);
   }
   ```

4. **Auto-scale calculation:**
   ```rust
   fn get_y_range(&self) -> (f32, f32) {
       let max_rpm = self.fan_rpm_series.iter()
           .fold(0.0_f32, |acc, &v| acc.max(v));
       let y_max = (max_rpm * 1.1).max(100.0); // 10% padding, minimum 100
       (0.0, y_max)
   }
   ```

5. **Render implementation:**
   - Use `theme.thermal_graph_light` and `theme.thermal_graph_dark` for gradient
   - Single graph (unlike charge panel which has voltage + current)

---

## Step 2: Create Thermal Info Panel

**File:** `src/tui/component/thermal_panel.rs`

Create a panel displaying current FAN RPM value with a title.

### Structure

```rust
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    framework::info::FrameworkInfo,
    tui::{component::Component, theme::Theme},
};

pub struct ThermalPanelComponent;
```

### Key Implementation Details

1. **Simple read-only display** (no adjustable controls for now)

2. **Display format:**
   ```
   ┌─ Thermal ─────────────┐
   │  Fan Speed    3200 RPM│
   └───────────────────────┘
   ```

3. **Render implementation:**
   ```rust
   impl Component for ThermalPanelComponent {
       fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
           let fan_rpm_text = match info.fan_rpm.as_ref().and_then(|rpms| rpms.first()) {
               Some(&rpm) => format!("{} RPM", rpm),
               None => "N/A".to_string(),
           };
           
           // Render key-value pair similar to charge_panel
           frame.render_widget(Paragraph::new("  Fan Speed"), key_area);
           frame.render_widget(
               Paragraph::new(fan_rpm_text).alignment(Alignment::Right),
               value_area
           );
       }
   }
   ```

---

## Step 3: Create Composite Panel

**File:** `src/tui/component/thermal_panels.rs`

Create a container that combines the graph and info panel.

### Structure

```rust
use ratatui::{
    crossterm::event::Event,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    app::AppEvent,
    framework::info::FrameworkInfo,
    tui::{
        component::{
            thermal_graph_panel::ThermalGraphPanelComponent,
            thermal_panel::ThermalPanelComponent,
            Component,
        },
        theme::Theme,
    },
};

pub struct ThermalPanelsComponent {
    graph_panel: ThermalGraphPanelComponent,
    thermal_panel: ThermalPanelComponent,
}
```

### Key Implementation Details

1. **Layout:** Vertical split with graph on top (filling space) and info panel on bottom (fixed height)
   ```rust
   let [graph_area, thermal_panel_area] =
       Layout::vertical([Constraint::Min(0), Constraint::Max(5)])
           .areas(inner_area);
   ```

2. **Border:** Rounded border with `theme.border` color (matches other panels)

3. **No input handling needed** (thermal panel is read-only for now)

---

## Step 4: Register Modules

**File:** `src/tui/component.rs`

Add module declarations and re-exports:

```rust
// Add after existing mod declarations
pub mod thermal_graph_panel;
pub mod thermal_panel;
pub mod thermal_panels;

// The thermal_panels module will be used by main.rs
```

---

## Step 5: Update Main Layout

**File:** `src/tui/component/main.rs`

### Changes Required

1. **Add import:**
   ```rust
   use crate::tui::component::thermal_panels::ThermalPanelsComponent;
   ```

2. **Add field to `MainComponent`:**
   ```rust
   pub struct MainComponent {
       privacy_panel: PrivacyPanelComponent,
       smbios_panel: SmbiosPanelComponent,
       thermal_panels: ThermalPanelsComponent,  // NEW
       pd_ports_panel: PdPortsPanelComponent,
       adjustable_panels: Vec<Box<dyn AdjustableComponent>>,
       selected_panel: Option<usize>,
   }
   ```

3. **Initialize in `new()`:**
   ```rust
   Self {
       privacy_panel: PrivacyPanelComponent,
       smbios_panel: SmbiosPanelComponent,
       thermal_panels: ThermalPanelsComponent::new(),  // NEW
       pd_ports_panel: PdPortsPanelComponent::new(),
       adjustable_panels,
       selected_panel: None,
   }
   ```

4. **Update `render()` layout:**
   
   Current:
   ```rust
   let [pd_ports_panel_area] = Layout::vertical([Constraint::Min(0)]).areas(bottom_area);
   ```
   
   New:
   ```rust
   // Split bottom area: thermal panels (40%) | PD ports (60%)
   let [thermal_panels_area, pd_ports_panel_area] =
       Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
           .areas(bottom_area);
   
   // Render thermal panels
   self.thermal_panels.render(frame, thermal_panels_area, theme, info);
   ```

---

## Step 6: Add Theme Colors

**File:** `src/tui/theme.rs`

### Changes Required

1. **Add fields to `Theme` struct:**
   ```rust
   pub struct Theme {
       // ... existing fields ...
       pub thermal_graph_light: Color,
       pub thermal_graph_dark: Color,
   }
   ```

2. **Add values to each theme variant** (15 themes total):
   - `framework()`: Use a distinct color (e.g., orange/red tones for "heat")
   - Example for Framework theme:
     ```rust
     thermal_graph_light: Color::from_str("#FF7043").unwrap(),  // Deep Orange 400
     thermal_graph_dark: Color::from_str("#BF360C").unwrap(),   // Deep Orange 900
     ```

3. **Theme color suggestions** (heat-related palette):
   - Framework: `#FF7043` / `#BF360C` (deep orange)
   - Dark themes: Orange/red gradients
   - Light themes: Lighter orange variants
   - Monochrome: Use existing text colors

---

## File Summary

| File | Action | Description |
|------|--------|-------------|
| `src/tui/component/thermal_graph_panel.rs` | Create | FAN RPM graph with auto-scaling |
| `src/tui/component/thermal_panel.rs` | Create | Current FAN RPM display |
| `src/tui/component/thermal_panels.rs` | Create | Container combining graph + panel |
| `src/tui/component.rs` | Modify | Add 3 module declarations |
| `src/tui/component/main.rs` | Modify | Add field, update layout |
| `src/tui/theme.rs` | Modify | Add 2 color fields to struct + all 15 variants |

---

## Visual Layout (After Implementation)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              Title Bar                                  │
├────────────────────────────────────────┬────────────────────────────────┤
│                                        │                                │
│         Charge Panels (60%)            │      Right Panels (40%)        │
│   ┌──────────────────────────────┐     │   ┌────────────────────────┐   │
│   │  Voltage/Current Graph       │     │   │  Brightness Panel      │   │
│   │                              │     │   └────────────────────────┘   │
│   │                              │     │   ┌────────────────────────┐   │
│   │  Charge Info Panel           │     │   │  Privacy | SMBIOS      │   │
│   └──────────────────────────────┘     │   └────────────────────────┘   │
├─────────────────────┬──────────────────┴────────────────────────────────┤
│                     │                                                   │
│  Thermal Panels     │              PD Ports Panel (60%)                 │
│      (40%)          │                                                   │
│  ┌───────────────┐  │   ┌───────────────────────────────────────────┐   │
│  │  FAN RPM      │  │   │  Port 1  │  Port 2  │  Port 3  │  Port 4  │   │
│  │  Graph        │  │   │  ...     │  ...     │  ...     │  ...     │   │
│  │               │  │   └───────────────────────────────────────────┘   │
│  │  Fan: 3200RPM │  │                                                   │
│  └───────────────┘  │                                                   │
├─────────────────────┴───────────────────────────────────────────────────┤
│                              Footer                                     │
└─────────────────────────────────────────────────────────────────────────┘
```
