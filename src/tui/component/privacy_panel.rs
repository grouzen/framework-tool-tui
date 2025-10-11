use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
};
use tui_framework_experiment::toggle_switch::{State, ToggleSwitch};

use crate::{
    framework::info::FrameworkInfo,
    tui::{component::Component, theme::Theme},
};

pub struct PrivacyPanelComponent;

impl PrivacyPanelComponent {
    fn render_mic(&self, frame: &mut Frame, area: Rect, info: &FrameworkInfo) {
        let toggle_state = if info.is_microphone_enabled {
            State::On
        } else {
            State::Off
        };
        let toggle = &ToggleSwitch::new("Microphone", toggle_state);

        frame.render_widget(toggle, area);
    }

    fn render_camera(&self, frame: &mut Frame, area: Rect, info: &FrameworkInfo) {
        let toggle_state = if info.is_camera_enabled {
            State::On
        } else {
            State::Off
        };
        let toggle = &ToggleSwitch::new("Camera", toggle_state);

        frame.render_widget(toggle, area);
    }
}

impl Component for PrivacyPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .title(" Privacy ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .border_type(BorderType::Rounded);

        let [mic_area, camera_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .spacing(1)
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(block.inner(area));

        // Micrhophone
        self.render_mic(frame, mic_area, info);

        // Camera
        self.render_camera(frame, camera_area, info);

        // Render block
        frame.render_widget(block, area);
    }
}
