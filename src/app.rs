use framework_lib::chromium_ec::CrosEc;
use ratatui::{Terminal, prelude::Backend};
use std::time::Duration;

use crate::{
    framework::{Framework, info::FrameworkInfo},
    tui::Tui,
};

pub const APP_TITLE: &str = " Framework System ";
pub const FOOTER_HELP: &str = "[tab] switch panels  [enter] focus/apply  [esc] cancel [q] quit";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct App {
    framework: Framework,
    info: FrameworkInfo,
    running: bool,
    tui: Tui,
}

pub enum AppEvent {
    Quit,
    SetMaxChargeLimit(u8),
    SetFingerprintBrightness(u8),
    SetKeyboardBrightness(u8),
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let poll_interval = Duration::from_millis(1000);
        let ec = CrosEc::new();
        let framework = Framework::new(ec, poll_interval);
        let info = FrameworkInfo::default();
        let tui = Tui::new();

        Self {
            framework,
            info,
            running: true,
            tui,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        self.info = self.framework.poll();

        while self.running {
            if let Some(new_info) = self.framework.poll_if_needed() {
                self.info = new_info;
            }

            self.tui.render(terminal, &self.info)?;

            if let Some(event) = self.tui.handle_input()? {
                self.handle_event(event)?;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: AppEvent) -> color_eyre::Result<()> {
        match event {
            AppEvent::Quit => self.quit(),
            AppEvent::SetMaxChargeLimit(value) => {
                self.framework.set_max_charge_limit(value)?;
                self.info.max_charge_limit = Some(value);
            }
            AppEvent::SetFingerprintBrightness(percentage) => {
                self.framework.set_fp_brightness(percentage)?;
                self.info.fp_brightness_percentage = Some(percentage);
            }
            AppEvent::SetKeyboardBrightness(percentage) => {
                self.framework.set_kb_brightness(percentage);
                self.info.kb_brightness_percentage = Some(percentage);
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
