# Graph Implementation Plan

## Overview

Add mirrored dual graph for voltage and current using the bgraph library (btop-style). Create a charge graph panel component displaying battery voltage and charger current as mirrored graphs. Build a composite charge panels component with the dual graph (left, fills remaining space) and charge panel (right, fixed narrower width, aligned to right edge). All panels stick to the right side with the graph taking remaining left space. Remove viewport constraints for full-terminal layout.

## Implementation Steps

### 1. Add bgraph dependency

Add `bgraph = "0.1"` to root `Cargo.toml` dependencies section

### 2. Remove viewport size constraints

In `src/tui.rs` (lines 143-156):
- Eliminate the centered `Max(140×49)` constraints
- Replace with direct vertical split of `frame.area()` into:
  - Title: `Length(3)`
  - Main: `Fill(1)`
  - Footer: `Length(3)`
- Title and footer span full terminal width at top and bottom

### 3. Create charge graph panel component

Create `src/tui/component/charge_graph_panel.rs`:
- Implement `Component` trait named `ChargeGraphPanelComponent`
- Add two `VecDeque<f32>` buffers for voltage and current history
- Render using bgraph's dual graph widget with mirrored layout
- Apply `ColorGradient::three_point` (blue→orange→red) with `GradientMode::Position`
- Fixed voltage range: 0-20V
- Auto-scaled current range from buffer min/max
- Add `Min` width constraint (e.g., Min 40-50 columns) to ensure usability in narrow terminals

### 4. Create charge panels composite component

Create `src/tui/component/charge_panels.rs`:
- Owns `ChargeGraphPanelComponent` and `ChargePanelComponent`
- Implements `Component` and `AdjustableComponent` traits
- Allocates more than half available height to dual graph
- Keeps charge panel fixed at current vertical position using dynamic padding calculation: `(available_height - charge_panel_height) / 2`
- Uses horizontal `Layout`:
  - Graph: `Fill(1)` (left side)
  - Charge panel: `Max(~55 columns)` (right-aligned, no gap)

### 5. Adjust main component layout for right-aligned panels

In `src/tui/component/main.rs`:
- Modify layout to right-align all panels
- Use horizontal `Layout`:
  - Left content: `Fill(1)` or `Min` constraint for graph
  - Panels column: `Max(~55-60 columns)` (slightly narrower than current)
- Replace `ChargePanelComponent` field with `ChargePanels`
- Update module declarations in `src/tui/component.rs` for both:
  - `charge_graph_panel` module
  - `charge_panels` module

### 6. Update graph data with zero defaults

In `ChargeGraphPanelComponent`'s `render` method:
- Extract `charger_voltage` and `charger_current` from `FrameworkInfo`
- Convert mV→V and mA→A using `.unwrap_or(0.0)` for None values
- Push both to history buffers together (bounded to ~150-200 samples)
- Render:
  - Voltage graph on top with fixed 0-20V range
  - Current graph mirrored on bottom with auto-scaled range

## Design Decisions

### Layout Specifications

1. **Right panel column width**: Make panels slightly narrower (~55-60 columns) to give more space to the graph
2. **Graph fill behavior**: Use `Min` width constraint (40-50 columns) to ensure usability in narrow terminals
3. **Dual graph height scaling**: Graph takes more than half of available height, charge panel remains fixed at current vertical position
4. **Panel spacing**: No gap between graph area and right-aligned panel column

### Data Handling

1. **History buffer**: 150-200 samples for smooth visualization
2. **Y-axis scaling**:
   - Voltage: Fixed 0-20V range for stable visualization
   - Current: Auto-scaled from buffer min/max for better visibility
3. **Missing data**: Use 0.0 as default value when voltage or current is None
4. **Buffer synchronization**: Add voltage and current samples together to maintain time alignment

### Graph Appearance

1. **Gradient**: `ColorGradient::three_point` with blue→orange→red colors
2. **Gradient mode**: `GradientMode::Position` for btop-style appearance
3. **Layout**: Mirrored dual graph with voltage on top, current on bottom
4. **Rendering**: Braille mode for high-resolution smooth graphs

## Files to Create

1. `src/tui/component/charge_graph_panel.rs` - Graph panel component
2. `src/tui/component/charge_panels.rs` - Composite panel component

## Files to Modify

1. `Cargo.toml` - Add bgraph dependency
2. `src/tui.rs` - Remove viewport size constraints
3. `src/tui/component.rs` - Add module declarations
4. `src/tui/component/main.rs` - Replace ChargePanelComponent with ChargePanels, adjust layout
5. `src/tui/component/charge_panel.rs` - May need minor adjustments for integration

## Dependencies

- `bgraph = "0.1"` - For btop-style Braille graphing
- `ratatui = "0.29"` - Already present, compatible with bgraph

## Testing Considerations

1. Test with various terminal sizes (narrow, wide, tall, short)
2. Verify graph scales properly with terminal height
3. Confirm charge panel maintains fixed position and size
4. Validate data conversion (mV→V, mA→A)
5. Check gradient colors match btop-style appearance
6. Test with missing data (None values)
7. Verify panel navigation (Tab key) still works correctly
8. Test adjustable controls on charge panel still function
