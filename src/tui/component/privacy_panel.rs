use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders},
};
use tui_framework_experiment::toggle_switch::{State, ToggleSwitch};

use crate::{framework::FrameworkControls, tui::component::Component};

pub struct PrivacyPanelComponent;

impl PrivacyPanelComponent {
    fn render_mic(&self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let toggle_state = if controls.is_microphone_enabled {
            State::On
        } else {
            State::Off
        };
        let toggle = &ToggleSwitch::new("Microphone", toggle_state);

        frame.render_widget(toggle, area);
    }

    fn render_camera(&self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let toggle_state = if controls.is_camera_enabled {
            State::On
        } else {
            State::Off
        };
        let toggle = &ToggleSwitch::new("Camera", toggle_state);

        frame.render_widget(toggle, area);
    }
}

impl Component for PrivacyPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let block = Block::default()
            .title(" Privacy ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [mic_area, camera_area] = Layout::vertical([Constraint::Max(1), Constraint::Max(1)])
            .spacing(1)
            .horizontal_margin(2)
            .vertical_margin(1)
            .areas(block.inner(area));

        // Micrhophone
        self.render_mic(frame, mic_area, controls);

        // Camera
        self.render_camera(frame, camera_area, controls);

        // Render block
        frame.render_widget(block, area);
    }
}
