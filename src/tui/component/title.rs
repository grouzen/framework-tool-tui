use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{app::APP_TITLE, framework::info::FrameworkInfo, tui::component::Component};

pub struct TitleComponent;

impl Component for TitleComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, info: &FrameworkInfo) {
        let block = Block::default()
            .title(APP_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [
            smbios_version_area,
            charging_status_area,
            charge_percentage_area,
            max_charge_limit_area,
            fan_speed_area,
        ] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Max(15),
            Constraint::Max(6),
            Constraint::Max(13),
            Constraint::Min(18),
        ])
        .spacing(1)
        .areas(block.inner(area));

        // BIOS version
        if let Some(smbios_version) = &info.smbios_version {
            frame.render_widget(
                Paragraph::new(format!("[ v{} ]", smbios_version)),
                smbios_version_area,
            );
        }

        let charge_percentage = info.charge_percentage;
        let charge_style = match charge_percentage {
            Some(charge_percentage) if charge_percentage < 15 => Style::new().red(),
            _ => Style::new().green(),
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
                Paragraph::new(format!("[ Max: {}% ]", max_charge_limit)),
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

            frame.render_widget(Paragraph::new(format!("[ {} ]", text)), fan_speed_area);
        }

        frame.render_widget(block, area);
    }
}
