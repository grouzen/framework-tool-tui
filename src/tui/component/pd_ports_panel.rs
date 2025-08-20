use crate::{framework::FrameworkControls, tui::component::Component};
use framework_lib::power::{UsbChargingType, UsbPowerRoles};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

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
        info: &Option<framework_lib::power::UsbPdPowerInfo>,
    ) {
        let block = Block::default()
            .title(format!(" {} ", name))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);

        let [
            role_area,
            charging_type_area,
            voltage_now_area,
            voltage_max_area,
            current_limit_area,
            current_max_area,
            dual_role_area,
            max_power_area,
        ] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .margin(1)
        .areas(block.inner(area));

        frame.render_widget(block, area);

        let key_names = [
            "Role:",
            "Charging type:",
            "Voltage now:",
            "Voltage max:",
            "Current limit:",
            "Current max:",
            "Dual role:",
            "Max power:",
        ];

        let values = match info {
            Some(info) => [
                match info.role {
                    UsbPowerRoles::Disconnected => "Disconnected".to_string(),
                    UsbPowerRoles::Source => "Source".to_string(),
                    UsbPowerRoles::Sink => "Sink".to_string(),
                    UsbPowerRoles::SinkNotCharging => "Sink (Not Charging)".to_string(),
                },
                match info.charging_type {
                    UsbChargingType::None => "None".to_string(),
                    UsbChargingType::PD => "PD".to_string(),
                    UsbChargingType::TypeC => "Type-C".to_string(),
                    UsbChargingType::Proprietary => "Proprietary".to_string(),
                    UsbChargingType::Bc12Dcp => "BC1.2 DCP".to_string(),
                    UsbChargingType::Bc12Cdp => "BC1.2 CDP".to_string(),
                    UsbChargingType::Bc12Sdp => "BC1.2 SDP".to_string(),
                    UsbChargingType::Other => "Other".to_string(),
                    UsbChargingType::VBus => "VBUS".to_string(),
                    UsbChargingType::Unknown => "Unknown".to_string(),
                },
                format!("{:.1} V", info.meas.voltage_now as f32 / 1000.0),
                format!("{:.1} V", info.meas.voltage_max as f32 / 1000.0),
                format!("{} mA", info.meas.current_lim),
                format!("{} mA", info.meas.current_max),
                if info.dualrole {
                    "DRP".to_string()
                } else {
                    "Charger".to_string()
                },
                format!("{:.2} W", info.max_power as f32 / 1000.0),
            ],
            None => [
                "(N/A)".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
            ],
        };

        let rows = [
            role_area,
            charging_type_area,
            voltage_now_area,
            voltage_max_area,
            current_limit_area,
            current_max_area,
            dual_role_area,
            max_power_area,
        ];

        for i in 0..8 {
            let [key_area, value_area] =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(rows[i]);

            frame.render_widget(Paragraph::new(key_names[i]), key_area);
            frame.render_widget(Paragraph::new(values[i].as_str()), value_area);
        }
    }
}

impl Component for PdPortsPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let block = Block::default()
            .title(" PD ports ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(block.inner(area));

        let [left_back_area, left_front_area] =
            Layout::vertical([Constraint::Max(12), Constraint::Max(12)]).areas(left_area);
        let [right_back_area, right_front_area] =
            Layout::vertical([Constraint::Max(12), Constraint::Max(12)]).areas(right_area);

        let left_back = &controls.pd_ports[3];
        let left_front = &controls.pd_ports[2];
        let right_back = &controls.pd_ports[0];
        let right_front = &controls.pd_ports[1];

        self.render_port_block(frame, left_back_area, "Left back", left_back);
        self.render_port_block(frame, left_front_area, "Left front", left_front);
        self.render_port_block(frame, right_back_area, "Right back", right_back);
        self.render_port_block(frame, right_front_area, "Right front", right_front);

        frame.render_widget(block, area);
    }
}
