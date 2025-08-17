use std::time::Duration;

pub mod component;

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    prelude::Backend,
};

use crate::{
    app::AppEvent,
    framework::FrameworkControls,
    tui::component::{
        Component, footer::FooterComponent, main::MainComponent, title::TitleComponent,
    },
};

pub struct Tui {
    title: TitleComponent,
    main: MainComponent,
    footer: FooterComponent,
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
        }
    }

    pub fn handle_input(&mut self) -> color_eyre::Result<Option<AppEvent>> {
        let event = if event::poll(Duration::from_millis(50))? {
            let event = event::read()?;

            self.handle_input_internal(event)?
        } else {
            None
        };

        Ok(event)
    }

    fn handle_input_internal(&mut self, event: Event) -> color_eyre::Result<Option<AppEvent>> {
        let top_level_event = match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Some(AppEvent::Quit),
                _ => None,
            },
            _ => None,
        };

        Ok(top_level_event.or(self.main.handle_input(event)?))
    }

    pub fn render<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        controls: &FrameworkControls,
    ) -> color_eyre::Result<()> {
        terminal.draw(|frame| {
            let [title_area, main_area, footer_area] =
                Layout::vertical([Constraint::Max(3), Constraint::Min(0), Constraint::Max(3)])
                    .areas(frame.area());

            // Title
            self.title.render(frame, title_area, controls);

            // Main
            self.main.render(frame, main_area, controls);

            // Footer
            self.footer.render(frame, footer_area, controls);
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

        let app_event = tui.handle_input_internal(event);

        assert!(matches!(app_event, Ok(Some(AppEvent::Quit))))
    }
}
