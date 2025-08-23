use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{framework::info::FrameworkInfo, tui::component::Component};

pub struct SmbiosPanelComponent;

impl SmbiosPanelComponent {
    fn render_smbios_version(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        info: &FrameworkInfo,
    ) {
        let smbios_version_text = match &info.smbios_version {
            Some(smbios_version) => smbios_version,
            None => "N/A",
        };

        frame.render_widget(Paragraph::new("Version"), key_area);
        frame.render_widget(Paragraph::new(smbios_version_text), value_area);
    }

    fn render_smbios_release_date(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        info: &FrameworkInfo,
    ) {
        let smbios_release_date_text = match &info.smbios_release_date {
            Some(smbios_release_date) => smbios_release_date,
            None => "N/A",
        };

        frame.render_widget(Paragraph::new("Release date"), key_area);
        frame.render_widget(Paragraph::new(smbios_release_date_text), value_area);
    }

    fn render_smbios_vendor(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        info: &FrameworkInfo,
    ) {
        let smbios_vendor_text = match &info.smbios_vendor {
            Some(smbios_vendor) => smbios_vendor,
            None => "N/A",
        };

        frame.render_widget(Paragraph::new("Vendor"), key_area);
        frame.render_widget(Paragraph::new(smbios_vendor_text), value_area);
    }
}

impl Component for SmbiosPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, info: &FrameworkInfo) {
        let block = Block::default()
            .title(" BIOS ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [
            smbios_vendor_key_area,
            smbios_version_key_area,
            smbios_release_date_key_area,
        ] = Layout::vertical([Constraint::Max(1), Constraint::Max(1), Constraint::Max(1)])
            .areas(keys_block.inner(keys_area));
        let [
            smbios_vendor_value_area,
            smbios_version_value_area,
            smbios_release_date_value_area,
        ] = Layout::vertical([Constraint::Max(1), Constraint::Max(1), Constraint::Max(1)])
            .areas(values_block.inner(values_area));

        // Vendor
        self.render_smbios_vendor(
            frame,
            smbios_vendor_key_area,
            smbios_vendor_value_area,
            info,
        );

        // Version
        self.render_smbios_version(
            frame,
            smbios_version_key_area,
            smbios_version_value_area,
            info,
        );

        // Release date
        self.render_smbios_release_date(
            frame,
            smbios_release_date_key_area,
            smbios_release_date_value_area,
            info,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }
}
