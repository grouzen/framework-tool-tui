use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    framework::FrameworkControls,
    tui::component::{
        Component, brightness_panel::BrightnessPanelComponent, charge_panel::ChargePanelComponent,
        privacy_panel::PrivacyPanelComponent, smbios_panel::SmbiosPanelComponent,
    },
};

pub struct MainComponent {
    charge_panel: ChargePanelComponent,
    privacy_panel: PrivacyPanelComponent,
    smbios_panel: SmbiosPanelComponent,
    brightness_panel: BrightnessPanelComponent,
}

impl MainComponent {
    pub fn new() -> Self {
        Self {
            charge_panel: ChargePanelComponent,
            privacy_panel: PrivacyPanelComponent,
            smbios_panel: SmbiosPanelComponent,
            brightness_panel: BrightnessPanelComponent,
        }
    }
}

impl Component for MainComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Min(0)]).areas(area);
        let [charge_panel_area, privacy_and_smbios_panels_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Max(7)]).areas(left_area);
        let [privacy_panel_area, smbios_panel_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Min(0)])
                .areas(privacy_and_smbios_panels_area);
        let [brightness_panel_area] = Layout::vertical([Constraint::Min(0)]).areas(right_area);

        // Charge panel
        self.charge_panel.render(frame, charge_panel_area, controls);

        // Privacy panel
        self.privacy_panel
            .render(frame, privacy_panel_area, controls);

        // SMBIOS panel
        self.smbios_panel.render(frame, smbios_panel_area, controls);

        // Brightness panel
        self.brightness_panel
            .render(frame, brightness_panel_area, controls);
    }
}
