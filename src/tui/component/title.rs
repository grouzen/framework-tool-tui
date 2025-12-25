use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::APP_TITLE,
    framework::info::FrameworkInfo,
    tui::{component::Component, theme::Theme},
};

pub struct TitleComponent {
    theme_name: String,
}

impl TitleComponent {
    pub fn new() -> Self {
        Self {
            theme_name: String::new(),
        }
    }

    pub fn set_theme_name(&mut self, name: String) {
        self.theme_name = name;
    }
}

impl Component for TitleComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .title(APP_TITLE)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .border_type(BorderType::Rounded);

        let [smbios_version_area, charging_status_area, charge_percentage_area, max_charge_limit_area, fan_speed_area, theme_area] =
            Layout::horizontal([
                Constraint::Max(10),
                Constraint::Max(15),
                Constraint::Max(6),
                Constraint::Max(13),
                Constraint::Min(18),
                Constraint::Max(19),
            ])
            .spacing(1)
            .areas(block.inner(area));

        // BIOS version
        if let Some(smbios_version) = &info.smbios_version {
            frame.render_widget(
                Paragraph::new(format!("[ v{} ]", smbios_version))
                    .style(Style::default().fg(theme.informative_text)),
                smbios_version_area,
            );
        }

        let charge_percentage = info.charge_percentage;
        let charge_style = match charge_percentage {
            Some(charge_percentage) if charge_percentage < 15 => {
                Style::default().fg(theme.indication_warning)
            }
            _ => Style::default().fg(theme.indication_ok),
        };

        // Charging status
        frame.render_widget(
            Paragraph::new(format!("[ {}", info.charging_status)).style(charge_style),
            charging_status_area,
        );

        // Charge percentage
        if let Some(charge_percentage) = info.charge_percentage {
            frame.render_widget(
                Paragraph::new(format!("{}% ]", charge_percentage)).style(charge_style),
                charge_percentage_area,
            );
        }

        // Max charge limit
        if let Some(max_charge_limit) = info.max_charge_limit {
            frame.render_widget(
                Paragraph::new(format!("[ Max: {}% ]", max_charge_limit))
                    .style(Style::default().fg(theme.informative_text)),
                max_charge_limit_area,
            );
        }

        // FAN speed
        if let Some(fan_rpm) = &info.fan_rpm {
            let text = fan_rpm
                .iter()
                .enumerate()
                .map(|(n, rpm)| format!("FAN{}: {} RPM", n + 1, rpm))
                .collect::<Vec<String>>()
                .join(", ");

            frame.render_widget(
                Paragraph::new(format!("[ {} ]", text))
                    .style(Style::default().fg(theme.informative_text)),
                fan_speed_area,
            );
        }

        // Theme name
        if !self.theme_name.is_empty() {
            frame.render_widget(
                Paragraph::new(format!("【←b {} n→】", self.theme_name))
                    .style(Style::default().fg(theme.highlighted_text))
                    .alignment(Alignment::Right),
                theme_area,
            );
        }

        frame.render_widget(block, area);
    }
}
