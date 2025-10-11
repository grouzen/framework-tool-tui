pub mod component;
pub mod control;
pub mod theme;

use ratatui::{
    Terminal,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Flex, Layout},
    prelude::Backend,
};

use crate::{
    app::AppEvent,
    framework::info::FrameworkInfo,
    tui::{
        component::{
            Component, footer::FooterComponent, main::MainComponent, title::TitleComponent,
        },
        theme::Theme,
    },
};

pub struct Tui {
    title: TitleComponent,
    main: MainComponent,
    footer: FooterComponent,
    theme: Theme,
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

impl Tui {
    pub fn new() -> Self {
        Self {
            title: TitleComponent,
            main: MainComponent::new(),
            footer: FooterComponent,
            theme: Theme::default(),
        }
    }

    pub fn handle_input(&mut self, event: Event) -> color_eyre::Result<Option<AppEvent>> {
        let top_level_event = match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Some(AppEvent::Quit),
                _ => None,
            },
            _ => None,
        };

        Ok(top_level_event.or(self.main.handle_input(event)))
    }

    pub fn render<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        info: &FrameworkInfo,
    ) -> color_eyre::Result<()> {
        terminal.draw(|frame| {
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
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

    use crate::{app::AppEvent, tui::Tui};

    #[test]
    fn handle_input_internal_quit_event() {
        let mut tui = Tui::new();
        let event = Event::Key(KeyEvent::from(KeyCode::Char('q')));

        let app_event = tui.handle_input(event);

        assert!(matches!(app_event, Ok(Some(AppEvent::Quit))))
    }
}
