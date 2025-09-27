use framework_lib::chromium_ec::CrosEc;
use ratatui::{Terminal, prelude::Backend};
use std::time::Duration;

use crate::{
    event::{Event, EventLoop},
    framework::{Framework, info::FrameworkInfo},
    tui::Tui,
};

pub const APP_TITLE: &str = " Framework System ";
pub const FOOTER_HELP: &str = "[tab] switch panels [up/down] scroll [enter] edit/apply [left/right] adjust value [esc] cancel [q] quit";
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

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        let mut event_loop = EventLoop::new();

        // Pre-fetch framework info
        self.info = self.framework.poll();

        event_loop.run(Duration::from_millis(1000));

        while self.running {
            self.tui.render(terminal, &self.info)?;

            match event_loop.next().await? {
                Event::Tick => {
                    self.info = self.framework.poll();
                }
                Event::Input(event) => {
                    if let Some(app_event) = self.tui.handle_input(event)? {
                        self.handle_event(app_event)?;
                    }
                }
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
