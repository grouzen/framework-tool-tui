use crate::{
    framework::info::{FrameworkInfo, PdPortInfo},
    tui::{component::Component, theme::Theme},
};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
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
        theme: &Theme,
        info: &Option<PdPortInfo>,
    ) {
        let block = Block::default()
            .title(format!(" {} ", name))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        if let Some(info) = info {
            let [key_area, value_area] =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                    .margin(1)
                    .spacing(1)
                    .areas(block.inner(area));

            let keys_block = Block::default().borders(Borders::NONE);
            let values_block = Block::default().borders(Borders::NONE);

            let [role_key_area, dualrole_key_area, charging_type_key_area, voltage_now_key_area, voltage_max_key_area, current_limit_key_area, current_max_key_area, max_power_key_area] =
                Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .flex(Flex::Legacy)
                .areas(keys_block.inner(key_area));
            let [role_value_area, dualrole_value_area, charging_type_value_area, voltage_now_value_area, voltage_max_value_area, current_limit_value_area, current_max_value_area, max_power_value_area] =
                Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .flex(Flex::Legacy)
                .areas(values_block.inner(value_area));

            self.render_role(frame, role_key_area, role_value_area, theme, info);
            self.render_dualrole(frame, dualrole_key_area, dualrole_value_area, theme, info);
            self.render_charging_type(
                frame,
                charging_type_key_area,
                charging_type_value_area,
                theme,
                info,
            );
            self.render_voltage_now(
                frame,
                voltage_now_key_area,
                voltage_now_value_area,
                theme,
                info,
            );
            self.render_voltage_max(
                frame,
                voltage_max_key_area,
                voltage_max_value_area,
                theme,
                info,
            );
            self.render_current_limit(
                frame,
                current_limit_key_area,
                current_limit_value_area,
                theme,
                info,
            );
            self.render_current_max(
                frame,
                current_max_key_area,
                current_max_value_area,
                theme,
                info,
            );
            self.render_max_power(frame, max_power_key_area, max_power_value_area, theme, info);
        }

        frame.render_widget(block, area);
    }

    fn render_role(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Role"), key_area);
        frame.render_widget(
            Paragraph::new(info.role.as_str()).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_dualrole(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Dual role"), key_area);
        frame.render_widget(
            Paragraph::new(info.dualrole.as_str())
                .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_charging_type(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Charging type"), key_area);
        frame.render_widget(
            Paragraph::new(info.charging_type.as_str())
                .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_voltage_now(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Voltage now"), key_area);
        frame.render_widget(
            Paragraph::new(format!("{:.1} V", info.voltage_now))
                .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_voltage_max(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Voltage max"), key_area);
        frame.render_widget(
            Paragraph::new(format!("{:.1} V", info.voltage_max))
                .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_current_limit(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Current limit"), key_area);
        frame.render_widget(
            Paragraph::new(format!("{} mA", info.current_limit))
                .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_current_max(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Current max"), key_area);
        frame.render_widget(
            Paragraph::new(format!("{} mA", info.current_max))
                .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_max_power(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &PdPortInfo,
    ) {
        frame.render_widget(Paragraph::new("Max power"), key_area);
        frame.render_widget(
            Paragraph::new(format!(
                "{}.{} W",
                info.max_power / 1000,
                info.max_power % 1000
            ))
            .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }
}

impl Component for PdPortsPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .title(" PD ports ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .border_type(BorderType::Rounded);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(block.inner(area));

        let [left_back_area, left_front_area] =
            Layout::vertical([Constraint::Min(5), Constraint::Min(0)])
                .margin(1)
                .areas(left_area);
        let [right_back_area, right_front_area] =
            Layout::vertical([Constraint::Min(5), Constraint::Min(0)])
                .margin(1)
                .areas(right_area);

        self.render_port_block(
            frame,
            left_back_area,
            "Left back",
            theme,
            &info.pd_ports.left_back,
        );
        self.render_port_block(
            frame,
            left_front_area,
            "Left front",
            theme,
            &info.pd_ports.left_front,
        );
        self.render_port_block(
            frame,
            right_back_area,
            "Right back",
            theme,
            &info.pd_ports.right_back,
        );
        self.render_port_block(
            frame,
            right_front_area,
            "Right front",
            theme,
            &info.pd_ports.right_front,
        );

        frame.render_widget(block, area);
    }
}
