use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    framework::info::FrameworkInfo,
    tui::{component::Component, theme::Theme},
};

pub struct PrivacyPanelComponent;

impl PrivacyPanelComponent {
    fn render_mic(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let toggle = if info.is_microphone_enabled {
            Paragraph::new("ON").style(Style::default().fg(theme.indication_ok))
        } else {
            Paragraph::new("OFF").style(Style::default().fg(theme.indication_warning))
        };

        frame.render_widget(Paragraph::new("Microphone"), key_area);
        frame.render_widget(toggle, value_area);
    }

    fn render_camera(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let toggle = if info.is_camera_enabled {
            Paragraph::new("ON").style(Style::default().fg(theme.indication_ok))
        } else {
            Paragraph::new("OFF").style(Style::default().fg(theme.indication_warning))
        };

        frame.render_widget(Paragraph::new("Camera"), key_area);
        frame.render_widget(toggle, value_area);
    }
}

impl Component for PrivacyPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .title(" Privacy ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .border_type(BorderType::Rounded);

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .horizontal_margin(2)
                .spacing(1)
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [mic_key_area, camera_key_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .spacing(1)
                .areas(keys_block.inner(keys_area));
        let [mic_value_area, camera_value_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .spacing(1)
                .areas(values_block.inner(values_area));

        // Micrhophone
        self.render_mic(frame, mic_key_area, mic_value_area, theme, info);

        // Camera
        self.render_camera(frame, camera_key_area, camera_value_area, theme, info);

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }
}
