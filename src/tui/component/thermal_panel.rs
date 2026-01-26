use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    framework::info::{FrameworkInfo, TempSensorInfo},
    tui::{component::Component, theme::Theme},
};

pub struct ThermalPanelComponent;

impl ThermalPanelComponent {
    fn render_fan_speed(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let fan_rpm_text = match info.fan_rpm.as_ref().and_then(|rpms| rpms.first()) {
            Some(&rpm) => format!("{} RPM", rpm),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Fan Speed"), key_area);
        frame.render_widget(
            Paragraph::new(fan_rpm_text).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_temp_sensor(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        sensor: &TempSensorInfo,
    ) {
        let temp_text = match sensor.temp_celsius {
            Some(temp) => format!("{} °C", temp),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new(format!("{}", sensor.name)), key_area);
        frame.render_widget(
            Paragraph::new(temp_text).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }
}

impl Component for ThermalPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let sensor_count = info.temp_sensors.len();
        // 1 row for fan speed + 1 row per temp sensor + 2 for vertical margin
        let panel_height = 1 + sensor_count + 2;

        let [area] = Layout::vertical([Constraint::Max(panel_height as u16)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::default()
            .title(" Thermal ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border));

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        // Create constraints: 1 for fan speed + 1 for each temp sensor
        let vertical_constraints = vec![Constraint::Length(1); info.temp_sensors.len() + 1];

        let key_areas = Layout::vertical(&vertical_constraints)
            .horizontal_margin(1)
            .split(keys_block.inner(keys_area));
        let value_areas = Layout::vertical(&vertical_constraints)
            .horizontal_margin(1)
            .split(values_block.inner(values_area));

        // Render fan speed in the first row
        if let (Some(&key_area), Some(&value_area)) = (key_areas.first(), value_areas.first()) {
            self.render_fan_speed(frame, key_area, value_area, theme, info);
        }

        // Render each temp sensor in subsequent rows
        for (i, sensor) in info.temp_sensors.iter().enumerate() {
            if let (Some(&key_area), Some(&value_area)) =
                (key_areas.get(i + 1), value_areas.get(i + 1))
            {
                self.render_temp_sensor(frame, key_area, value_area, theme, sensor);
            }
        }

        frame.render_widget(block, area);
    }
}
