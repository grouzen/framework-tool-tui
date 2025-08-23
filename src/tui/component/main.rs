use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout, Rect},
};

use crate::{
    framework::info::FrameworkInfo,
    tui::component::{
        AdjustableComponent, Component, brightness_panel::BrightnessPanelComponent,
        charge_panel::ChargePanelComponent, pd_ports_panel::PdPortsPanelComponent,
        privacy_panel::PrivacyPanelComponent, smbios_panel::SmbiosPanelComponent,
    },
};

pub struct MainComponent {
    privacy_panel: PrivacyPanelComponent,
    smbios_panel: SmbiosPanelComponent,
    pd_ports_panel: PdPortsPanelComponent,
    adjustable_panels: Vec<Box<dyn AdjustableComponent>>,
    selected_panel: Option<usize>,
}

impl Default for MainComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl MainComponent {
    pub fn new() -> Self {
        let charge_panel = Box::new(ChargePanelComponent::new());
        let brightness_panel = Box::new(BrightnessPanelComponent::new());

        Self {
            privacy_panel: PrivacyPanelComponent,
            smbios_panel: SmbiosPanelComponent,
            pd_ports_panel: PdPortsPanelComponent::new(),
            adjustable_panels: vec![charge_panel, brightness_panel],
            selected_panel: None,
        }
    }

    fn switch_panels(&mut self) {
        let len = self.adjustable_panels.len();

        if let Some(selected_panel) = self.selected_panel {
            if selected_panel < len - 1 {
                let next = selected_panel + 1;

                self.adjustable_panels[selected_panel].panel().toggle();
                self.adjustable_panels[next].panel().toggle();

                self.selected_panel = Some(next);
            } else {
                self.adjustable_panels[selected_panel].panel().toggle();
                self.selected_panel = None;
            }
        } else {
            self.adjustable_panels[0].panel().toggle();
            self.selected_panel = Some(0);
        }
    }
}

impl Component for MainComponent {
    fn handle_input(&mut self, event: Event) -> Option<crate::app::AppEvent> {
        if let Event::Key(key) = event
            && key.code == KeyCode::Tab
        {
            self.switch_panels();
        }

        self.adjustable_panels
            .iter_mut()
            .find_map(|panel| panel.handle_input(event.clone()))
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, info: &FrameworkInfo) {
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Min(0)]).areas(area);
        let [charge_panel_area, privacy_and_smbios_panels_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Max(7)]).areas(left_area);
        let [privacy_panel_area, smbios_panel_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Min(0)])
                .areas(privacy_and_smbios_panels_area);
        let [brightness_panel_area, pd_ports_panel_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(2)]).areas(right_area);

        // Charge panel
        self.adjustable_panels[0].render(frame, charge_panel_area, info);

        // Privacy panel
        self.privacy_panel.render(frame, privacy_panel_area, info);

        // SMBIOS panel
        self.smbios_panel.render(frame, smbios_panel_area, info);

        // Brightness panel (top of right_area)
        self.adjustable_panels[1].render(frame, brightness_panel_area, info);

        // PD Ports panel (bottom of right_area)
        self.pd_ports_panel.render(frame, pd_ports_panel_area, info);
    }
}
