use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{app::APP_TITLE, framework::FrameworkControls, tui::component::Component};

pub struct TitleComponent;

impl Component for TitleComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let block = Block::default()
            .title(APP_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [
            smbios_version_area,
            charging_status_area,
            charge_percentage_area,
            max_charge_limit_area,
        ] = Layout::horizontal([
            Constraint::Max(6),
            Constraint::Max(13),
            Constraint::Max(4),
            Constraint::Max(9),
        ])
        .horizontal_margin(2)
        .spacing(2)
        .areas(block.inner(area));

        // BIOS version
        if let Some(smbios_version) = controls.smbios_version() {
            frame.render_widget(
                Paragraph::new(format!("v{}", smbios_version)),
                smbios_version_area,
            );
        }

        let charge_percentage = controls.charge_percentage();
        let charge_style = match charge_percentage {
            Some(charge_percentage) if charge_percentage < 15 => Style::new().red(),
            _ => Style::new().green(),
        };

        // Charging status
        frame.render_widget(
            Paragraph::new(controls.charging_status()).style(charge_style),
            charging_status_area,
        );

        // Charge percentage
        if let Some(charge_percentage) = controls.charge_percentage() {
            frame.render_widget(
                Paragraph::new(format!("{}%", charge_percentage)).style(charge_style),
                charge_percentage_area,
            );
        }

        // Max charge limit
        if let Some(max_charge_limit) = controls.max_charge_limit() {
            frame.render_widget(
                Paragraph::new(format!("Max: {}%", max_charge_limit)),
                max_charge_limit_area,
            );
        }

        frame.render_widget(block, area);
    }
}
