pub mod component;
pub mod control;
pub mod theme;

use std::sync::Arc;

use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Flex, Layout},
    prelude::Backend,
    style::Style,
    text::Text,
    widgets::Block,
    Frame, Terminal,
};
use tui_popup::Popup;

use crate::{
    app::AppEvent,
    framework::{fingerprint::Fingerprint, info::FrameworkInfo},
    tui::{
        component::{
            footer::FooterComponent, main::MainComponent, title::TitleComponent, Component,
        },
        theme::Theme,
    },
};

pub struct Tui {
    title: TitleComponent,
    main: MainComponent,
    footer: FooterComponent,
    theme: Theme,
    error_message: Option<String>,
}

impl Tui {
    pub fn new(fingerprint: Arc<Fingerprint>, info: &FrameworkInfo) -> Self {
        Self {
            title: TitleComponent,
            main: MainComponent::new(fingerprint, info),
            footer: FooterComponent,
            theme: Theme::default(),
            error_message: None,
        }
    }

    pub fn handle_input(&mut self, event: Event) -> color_eyre::Result<Option<AppEvent>> {
        let top_level_event = match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Some(AppEvent::Quit),
                KeyCode::Esc if self.error_message.is_some() => {
                    self.error_message = None;
                    None
                }
                _ => None,
            },
            _ => None,
        };

        match self.error_message {
            Some(_) => Ok(top_level_event),
            None => Ok(top_level_event.or(self.main.handle_input(event))),
        }
    }

    pub fn render<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        info: &FrameworkInfo,
    ) -> color_eyre::Result<()> {
        terminal.draw(|frame| {
            let block = Block::default().style(Style::default().bg(self.theme.background));
            frame.render_widget(block, frame.area());

            let area = frame.area();
            let [area] = Layout::vertical([Constraint::Max(49)])
                .flex(Flex::Center)
                .areas(area);
            let [area] = Layout::horizontal([Constraint::Max(140)])
                .flex(Flex::Center)
                .areas(area);

            let [title_area, main_area, footer_area] =
                Layout::vertical([Constraint::Max(3), Constraint::Max(44), Constraint::Max(3)])
                    .flex(Flex::Center)
                    .areas(area);

            // Title
            self.title.render(frame, title_area, &self.theme, info);

            // Main
            self.main.render(frame, main_area, &self.theme, info);

            // Footer
            self.footer.render(frame, footer_area, &self.theme, info);

            // Error popup if error is set
            self.render_error_popup(frame);
        })?;

        Ok(())
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    fn render_error_popup(&self, frame: &mut Frame) {
        if let Some(message) = &self.error_message {
            let mut text = Text::default();
            let message = format!(" {} ", message);

            text.push_line("");
            text.push_line(message.as_str());
            text.push_line("");

            let popup = Popup::new(text)
                .title(" Error ")
                .style(
                    Style::default()
                        .bg(self.theme.background)
                        .fg(self.theme.indication_warning),
                )
                .border_style(Style::default().fg(self.theme.border));

            frame.render_widget(&popup, frame.area());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

    use crate::{
        app::AppEvent,
        framework::{fingerprint::Fingerprint, info::FrameworkInfo},
        tui::Tui,
    };

    #[test]
    fn handle_input_internal_quit_event() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let mut tui = Tui::new(fingerprint, &info);
        let event = Event::Key(KeyEvent::from(KeyCode::Char('q')));

        let app_event = tui.handle_input(event);

        assert!(matches!(app_event, Ok(Some(AppEvent::Quit))))
    }
}
