use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::{FOOTER_HELP, VERSION},
    framework::info::FrameworkInfo,
    tui::component::Component,
};

pub struct FooterComponent;

impl Component for FooterComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, _info: &FrameworkInfo) {
        let block = Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [help_area, version_area] =
            ratatui::layout::Layout::horizontal([Constraint::Min(1), Constraint::Max(12)])
                .horizontal_margin(1)
                .areas(block.inner(area));

        frame.render_widget(Paragraph::new(FOOTER_HELP), help_area);
        frame.render_widget(
            Paragraph::new(format!("v{}", VERSION)).alignment(ratatui::prelude::Alignment::Right),
            version_area,
        );

        frame.render_widget(block, area);
    }
}
