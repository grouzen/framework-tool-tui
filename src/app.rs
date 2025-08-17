use framework_lib::chromium_ec::CrosEc;
use ratatui::{Terminal, prelude::Backend};
use std::time::Duration;

use crate::{framework::Framework, tui::Tui};

pub const APP_TITLE: &str = " Framework System ";
pub const FOOTER_HELP: &str = "[tab] switch panels  [enter] focus/apply  [esc] cancel [q] quit";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct App {
    framework: Framework,
    running: bool,
    tui: Tui,
}

pub enum AppEvent {
    Quit,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let poll_interval = Duration::from_millis(2000);
        let ec = CrosEc::new();
        let framework = Framework::new(ec, poll_interval);
        let tui = Tui::new();

        Self {
            framework,
            running: true,
            tui,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        let mut controls = self.framework.poll();

        while self.running {
            if let Some(new_controls) = self.framework.poll_if_needed() {
                controls = new_controls;
            }

            self.tui.render(terminal, &controls)?;

            if let Some(event) = self.tui.handle_input()? {
                self.handle_event(event);
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Quit => self.quit(),
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
