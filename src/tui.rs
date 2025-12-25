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
    config::Config,
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
    config: Config,
}

impl Tui {
    pub fn new(
        fingerprint: Arc<Fingerprint>,
        info: &FrameworkInfo,
        config: Config,
    ) -> color_eyre::Result<Self> {
        let theme = Theme::from_variant(config.theme);

        Ok(Self {
            title: TitleComponent::new(theme.variant),
            main: MainComponent::new(fingerprint, info),
            footer: FooterComponent,
            theme,
            error_message: None,
            config,
        })
    }

    pub fn next_theme(&mut self) {
        let next_variant = self.config.theme.next();
        self.theme = Theme::from_variant(next_variant);
        if let Err(e) = self.config.set_theme(next_variant) {
            self.set_error(format!("Failed to save theme: {}", e));
        }
    }

    pub fn previous_theme(&mut self) {
        let prev_variant = self.config.theme.previous();
        self.theme = Theme::from_variant(prev_variant);
        if let Err(e) = self.config.set_theme(prev_variant) {
            self.set_error(format!("Failed to save theme: {}", e));
        }
    }

    pub fn current_theme_name(&self) -> &'static str {
        self.config.theme.name()
    }

    pub fn handle_input(&mut self, event: Event) -> color_eyre::Result<Option<AppEvent>> {
        let top_level_event = match &event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Some(AppEvent::Quit),
                KeyCode::Char('b') => {
                    self.previous_theme();
                    None
                }
                KeyCode::Char('n') => {
                    self.next_theme();
                    None
                }
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
            let block = Block::default().style(
                Style::default()
                    .bg(self.theme.background)
                    .fg(self.theme.text),
            );
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
            self.title
                .set_theme_name(self.current_theme_name().to_string());
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
        config::Config,
        framework::{fingerprint::Fingerprint, info::FrameworkInfo},
        tui::{theme::ThemeVariant, Tui},
    };

    #[test]
    fn handle_input_internal_quit_event() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();
        let event = Event::Key(KeyEvent::from(KeyCode::Char('q')));

        let app_event = tui.handle_input(event);

        assert!(matches!(app_event, Ok(Some(AppEvent::Quit))))
    }

    #[test]
    fn next_theme_cycles_forward() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        // Default theme should be Framework
        assert_eq!(tui.config.theme, ThemeVariant::Framework);

        // Cycle to next theme
        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::Alucard);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::Dracula);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinFrappe);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinLatte);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinMacchiato);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinMocha);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::GithubDark);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::GithubLight);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::MonokaiProDark);

        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::MonokaiProLight);

        // Should wrap back to Framework
        tui.next_theme();
        assert_eq!(tui.config.theme, ThemeVariant::Framework);
    }

    #[test]
    fn previous_theme_cycles_backward() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        // Default theme should be Framework
        assert_eq!(tui.config.theme, ThemeVariant::Framework);

        // Cycle to previous theme (should wrap to Gruvbox)
        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::MonokaiProLight);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::MonokaiProDark);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::GithubLight);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::GithubDark);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinMocha);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinMacchiato);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinLatte);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinFrappe);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::Dracula);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::Alucard);

        tui.previous_theme();
        assert_eq!(tui.config.theme, ThemeVariant::Framework);
    }

    #[test]
    fn current_theme_name_returns_correct_name() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        assert_eq!(tui.current_theme_name(), "Framework");

        tui.next_theme();
        assert_eq!(tui.current_theme_name(), "Alucard");

        tui.next_theme();
        assert_eq!(tui.current_theme_name(), "Dracula");

        tui.next_theme();
        assert_eq!(tui.current_theme_name(), "Catppuccin Frappe");
    }

    #[test]
    fn handle_input_n_switches_to_next_theme() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        assert_eq!(tui.config.theme, ThemeVariant::Framework);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('n')));
        let result = tui.handle_input(event);

        assert!(matches!(result, Ok(None)));
        assert_eq!(tui.config.theme, ThemeVariant::Alucard);
    }

    #[test]
    fn handle_input_b_switches_to_previous_theme() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        assert_eq!(tui.config.theme, ThemeVariant::Framework);

        let event = Event::Key(KeyEvent::from(KeyCode::Char('b')));
        let result = tui.handle_input(event);

        assert!(matches!(result, Ok(None)));
        assert_eq!(tui.config.theme, ThemeVariant::MonokaiProLight);
    }

    #[test]
    fn handle_input_left_without_ctrl_does_not_switch_theme() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        let initial_theme = tui.config.theme;
        let event = Event::Key(KeyEvent::from(KeyCode::Left));
        let _result = tui.handle_input(event);

        // Theme should remain unchanged
        assert_eq!(tui.config.theme, initial_theme);
    }

    #[test]
    fn handle_input_right_without_ctrl_does_not_switch_theme() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        let initial_theme = tui.config.theme;
        let event = Event::Key(KeyEvent::from(KeyCode::Right));
        let _result = tui.handle_input(event);

        // Theme should remain unchanged
        assert_eq!(tui.config.theme, initial_theme);
    }

    #[test]
    fn theme_switching_does_not_pass_event_to_main_component() {
        // This test ensures that 'b' and 'n' events are consumed by theme switching
        // and not passed down to child components
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        let initial_theme = tui.config.theme;

        // Send 'n' event
        let event = Event::Key(KeyEvent::from(KeyCode::Char('n')));
        let result = tui.handle_input(event);

        // Should return Ok(None) and change theme
        assert!(matches!(result, Ok(None)));
        assert_ne!(tui.config.theme, initial_theme);

        // Try switching back with 'b'
        let new_theme = tui.config.theme;
        let event = Event::Key(KeyEvent::from(KeyCode::Char('b')));
        let result = tui.handle_input(event);

        // Should return Ok(None) and change theme back
        assert!(matches!(result, Ok(None)));
        assert_eq!(tui.config.theme, initial_theme);
        assert_ne!(tui.config.theme, new_theme);
    }

    #[test]
    fn multiple_theme_switches_work_correctly() {
        let fingerprint = Arc::new(Fingerprint::percentage());
        let info = FrameworkInfo::default();
        let config = Config::default();
        let mut tui = Tui::new(fingerprint, &info, config).unwrap();

        // Start at Framework
        assert_eq!(tui.config.theme, ThemeVariant::Framework);

        // Switch forward 3 times with 'n'
        for _ in 0..3 {
            let event = Event::Key(KeyEvent::from(KeyCode::Char('n')));
            let result = tui.handle_input(event);
            assert!(matches!(result, Ok(None)));
        }
        // After 3 next: Framework -> Alucard -> Dracula -> CatppuccinFrappe
        assert_eq!(tui.config.theme, ThemeVariant::CatppuccinFrappe);

        // Switch backward once with 'b'
        let event = Event::Key(KeyEvent::from(KeyCode::Char('b')));
        let result = tui.handle_input(event);
        assert!(matches!(result, Ok(None)));
        assert_eq!(tui.config.theme, ThemeVariant::Dracula);
    }
}
