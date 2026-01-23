use std::sync::Arc;

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::{
    framework::{fingerprint::Fingerprint, info::FrameworkInfo},
    tui::{
        component::{
            brightness_panel::BrightnessPanelComponent, charge_panels::ChargePanelsComponent,
            pd_ports_panel::PdPortsPanelComponent, privacy_panel::PrivacyPanelComponent,
            smbios_panel::SmbiosPanelComponent, AdjustableComponent, Component,
        },
        theme::Theme,
    },
};

pub struct MainComponent {
    privacy_panel: PrivacyPanelComponent,
    smbios_panel: SmbiosPanelComponent,
    pd_ports_panel: PdPortsPanelComponent,
    adjustable_panels: Vec<Box<dyn AdjustableComponent>>,
    selected_panel: Option<usize>,
}

impl MainComponent {
    pub fn new(finterprint: Arc<Fingerprint>, info: &FrameworkInfo) -> Self {
        let mut adjustable_panels: Vec<Box<dyn AdjustableComponent>> = Vec::new();
        let charge_panels = Box::new(ChargePanelsComponent::new());

        adjustable_panels.push(charge_panels);

        if Self::is_brightness_supported(info) {
            let brightness_panel = Box::new(BrightnessPanelComponent::new(finterprint));

            adjustable_panels.push(brightness_panel);
        }

        Self {
            privacy_panel: PrivacyPanelComponent,
            smbios_panel: SmbiosPanelComponent,
            pd_ports_panel: PdPortsPanelComponent::new(),
            adjustable_panels,
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

    fn render_privacy_and_smbios_panels(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let [privacy_panel_area, smbios_panel_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Min(0)]).areas(area);

        // Privacy panel
        self.privacy_panel
            .render(frame, privacy_panel_area, theme, info);

        // SMBIOS panel
        self.smbios_panel
            .render(frame, smbios_panel_area, theme, info);
    }

    fn is_brightness_supported(info: &FrameworkInfo) -> bool {
        // NOTE: modifiying FP and KB brightness is not supported on FW 12
        info.platform != Some(framework_lib::smbios::Platform::Framework12IntelGen13)
    }
}

impl Component for MainComponent {
    fn handle_input(&mut self, event: Event) -> Option<crate::app::AppEvent> {
        if let Event::Key(key) = &event {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab {
                self.switch_panels();
            }
        }

        self.adjustable_panels
            .iter_mut()
            .find_map(|panel| panel.handle_input(event.clone()))
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        // Split main area: left content area and right panels column
        let [left_area, right_panels_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Max(60)]).areas(area);

        // Render charge panels (graph + charge panel) in the left area
        self.adjustable_panels[0].render(frame, left_area, theme, info);

        // Split right panels area vertically
        // Show brightness panel only on supported platforms
        if Self::is_brightness_supported(info) {
            let [brightness_panel_area, privacy_and_smbios_panels_area, pd_ports_panel_area] =
                Layout::vertical([
                    Constraint::Min(7),
                    Constraint::Min(7),
                    Constraint::Fill(1),
                ])
                .areas(right_panels_area);

            // Brightness panel (top of right_area)
            self.adjustable_panels[1].render(frame, brightness_panel_area, theme, info);

            // Privacy and SMBIOS panels
            self.render_privacy_and_smbios_panels(
                frame,
                privacy_and_smbios_panels_area,
                theme,
                info,
            );

            // PD Ports panel (bottom of right_area)
            self.pd_ports_panel
                .render(frame, pd_ports_panel_area, theme, info);
        } else {
            let [privacy_and_smbios_panels_area, pd_ports_panel_area] =
                Layout::vertical([Constraint::Min(14), Constraint::Fill(1)])
                    .areas(right_panels_area);

            // Privacy and SMBIOS panels
            self.render_privacy_and_smbios_panels(
                frame,
                privacy_and_smbios_panels_area,
                theme,
                info,
            );

            // PD Ports panel (bottom of right_area)
            self.pd_ports_panel
                .render(frame, pd_ports_panel_area, theme, info);
        }
    }
}
