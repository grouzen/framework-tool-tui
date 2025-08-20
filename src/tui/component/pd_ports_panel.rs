use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};
use crate::{
    framework::FrameworkControls,
    tui::component::{Component, AdjustablePanel},
};

const PD_PORT_NAMES: [&str; 4] = [
    "Right Back",
    "Right Front",
    "Left Front",
    "Left Back",
];

/// Mapping indexes to desired layout positions:
/// | 0 (Right Back) | 1 (Right Front) |
/// | 3 (Left Back)  | 2 (Left Front)  |
const PORT_LAYOUT: [(usize, usize); 4] = [
    (0, 0), // [row, column] for Right Back
    (0, 1), // Right Front
    (1, 1), // Left Front
    (1, 0), // Left Back
];

pub struct PdPortsPanelComponent;

impl Default for PdPortsPanelComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl PdPortsPanelComponent {
    pub fn new() -> Self {
        PdPortsPanelComponent
    }

    fn render_port_block(
        &self,
        frame: &mut Frame,
        area: Rect,
        name: &str,
        info: &Result<framework_lib::power::UsbPdPowerInfo, framework_lib::chromium_ec::EcError>,
    ) {
        let content = match info {
            Ok(info) => {
                let role = match info.role {
                    framework_lib::power::UsbPowerRoles::Disconnected => "Disconnected",
                    framework_lib::power::UsbPowerRoles::Source => "Source",
                    framework_lib::power::UsbPowerRoles::Sink => "Sink",
                    framework_lib::power::UsbPowerRoles::SinkNotCharging => "Sink (Not Charging)",
                };
                let charging_type = match info.charging_type {
                    framework_lib::power::UsbChargingType::None => "None",
                    framework_lib::power::UsbChargingType::PD => "PD",
                    framework_lib::power::UsbChargingType::TypeC => "Type-C",
                    framework_lib::power::UsbChargingType::Proprietary => "Proprietary",
                    framework_lib::power::UsbChargingType::Bc12Dcp => "BC1.2 DCP",
                    framework_lib::power::UsbChargingType::Bc12Cdp => "BC1.2 CDP",
                    framework_lib::power::UsbChargingType::Bc12Sdp => "BC1.2 SDP",
                    framework_lib::power::UsbChargingType::Other => "Other",
                    framework_lib::power::UsbChargingType::VBus => "VBUS",
                    framework_lib::power::UsbChargingType::Unknown => "Unknown",
                };
                let meas = &info.meas;
                format!(
                    "Role:          {}\n\
Charging Type:  {}\n\
Voltage Now:    {:.1} V\n\
Voltage Max:    {:.1} V\n\
Current Lim:    {} mA\n\
Current Max:    {} mA\n\
Dual Role:      {}\n\
Max Power:      {:.2} W",
                    role,
                    charging_type,
                    meas.voltage_now as f32 / 1000.0,
                    meas.voltage_max as f32 / 1000.0,
                    meas.current_lim,
                    meas.current_max,
                    if info.dualrole { "DRP" } else { "Charger" },
                    info.max_power as f32 / 1000.0,
                )
            }
            Err(_) => String::from("(Error reading port status)"),
        };

        let block = Paragraph::new(content)
            .block(
                Block::default()
                    .title(format!(" {}", name))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(block, area);
    }
}

impl Component for PdPortsPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        // Outer grid: 2 rows, 2 columns
        let [row0, row1]: [Rect; 2] =
            Layout::vertical([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .margin(1)
                .areas(area);

        let mut ports_info = [None, None, None, None];
        for (idx, info) in controls.pd_ports.iter().take(4).enumerate() {
            ports_info[idx] = Some(info);
        }

        for (port_idx, &(row, col)) in PORT_LAYOUT.iter().enumerate() {
            let row_area = if row == 0 { row0 } else { row1 };
            let [col0, col1]: [Rect; 2] =
                Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                    .areas(row_area);

            let col_area = if col == 0 { col0 } else { col1 };
            if let Some(info) = ports_info[port_idx] {
                self.render_port_block(
                    frame,
                    col_area,
                    PD_PORT_NAMES[port_idx],
                    info,
                );
            }
        }
    }
}