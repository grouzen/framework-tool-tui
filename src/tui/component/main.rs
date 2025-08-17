use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout, Rect},
};

use crate::{
    framework::FrameworkControls,
    tui::component::{
        Component, SelectableComponent, brightness_panel::BrightnessPanelComponent,
        charge_panel::ChargePanelComponent, privacy_panel::PrivacyPanelComponent,
        smbios_panel::SmbiosPanelComponent,
    },
};

pub struct MainComponent {
    privacy_panel: PrivacyPanelComponent,
    smbios_panel: SmbiosPanelComponent,
    selectable_panels: Vec<Box<dyn SelectableComponent>>,
    selected_panel: Option<usize>,
}

impl MainComponent {
    pub fn new() -> Self {
        let charge_panel = Box::new(ChargePanelComponent::new());
        let brightness_panel = Box::new(BrightnessPanelComponent::new());

        Self {
            privacy_panel: PrivacyPanelComponent,
            smbios_panel: SmbiosPanelComponent,
            selectable_panels: vec![charge_panel, brightness_panel],
            selected_panel: None,
        }
    }

    fn switch_panels(&mut self) {
        let len = self.selectable_panels.len();

        if let Some(selected_panel) = self.selected_panel {
            if selected_panel < len - 1 {
                let next = selected_panel + 1;

                self.selectable_panels[selected_panel].toggle();
                self.selectable_panels[next].toggle();

                self.selected_panel = Some(next);
            } else {
                self.selectable_panels[selected_panel].toggle();
                self.selected_panel = None;
            }
        } else {
            self.selectable_panels[0].toggle();
            self.selected_panel = Some(0);
        }
    }
}

impl Component for MainComponent {
    fn handle_input(&mut self, event: Event) -> color_eyre::Result<Option<crate::app::AppEvent>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Tab => {
                    self.switch_panels();
                }
                _ => {}
            },
            _ => {}
        }

        Ok(None)
    }

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
        self.selectable_panels[0].render(frame, charge_panel_area, controls);

        // Privacy panel
        self.privacy_panel
            .render(frame, privacy_panel_area, controls);

        // SMBIOS panel
        self.smbios_panel.render(frame, smbios_panel_area, controls);

        // Brightness panel
        self.selectable_panels[1].render(frame, brightness_panel_area, controls);
    }
}
