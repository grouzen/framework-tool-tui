use color_eyre::eyre::Report;
use framework_lib::chromium_ec::{CrosEc, EcError, EcResponseStatus};
use ratatui::{prelude::Backend, Terminal};
use std::{sync::Arc, time::Duration};

use crate::{
    config::Config,
    event::{Event, EventLoop},
    framework::{fingerprint::Fingerprint, info::FrameworkInfo, EcErrorWrapper, Framework},
    tui::Tui,
};

pub const APP_TITLE: &str = " Framework System ";
pub const FOOTER_HELP: &str = "[Tab] Switch panels [Up/Down] Scroll [Enter] Edit/Apply [Left/Right] Adjust value [Esc] Cancel [q] Quit";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct App {
    framework: Framework,
    info: FrameworkInfo,
    running: bool,
    tui: Tui,
    config: Config,
}

pub enum AppEvent {
    Quit,
    SetMaxChargeLimit(u8),
    SetFingerprintBrightness(u8),
    SetKeyboardBrightness(u8),
    SetTickInterval(u64),
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let ec = CrosEc::new();
        let fingerprint = Arc::new(Fingerprint::new(&ec)?);
        // Pre-fetch framework info
        let mut framework = Framework::new(ec, fingerprint.clone());
        let info = framework.get_info();

        // Load config (or create default on first startup)
        let config = Config::load_or_create()?;
        let tui = Tui::new(fingerprint, &info, config.clone())?;

        Ok(Self {
            framework,
            info,
            running: true,
            tui,
            config,
        })
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        let mut event_loop = EventLoop::new();

        event_loop.run(Duration::from_millis(self.config.tick_interval_ms));
        self.tui
            .title
            .set_tick_interval(self.config.tick_interval_ms);

        while self.running {
            self.tui.render(terminal, &self.info)?;

            match event_loop.next().await? {
                Event::Tick => {
                    self.info = self.framework.get_info();
                }
                Event::Input(event) => {
                    if let Some(app_event) = self.tui.handle_input(event)? {
                        self.handle_event(app_event, &event_loop)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: AppEvent, event_loop: &EventLoop) -> color_eyre::Result<()> {
        match event {
            AppEvent::Quit => self.quit(),
            AppEvent::SetMaxChargeLimit(value) => {
                self.framework.set_max_charge_limit(value)?;
                self.info.max_charge_limit = Some(value);
            }
            AppEvent::SetFingerprintBrightness(percentage) => {
                match self.framework.set_fp_brightness(percentage) {
                    Err(report) => match report.downcast::<EcErrorWrapper>() {
                        Ok(EcErrorWrapper(EcError::Response(EcResponseStatus::InvalidVersion))) => {
                            self.tui.set_error(
                                "Couldn't set fingerprint brightness. Please, update your BIOS."
                                    .to_string(),
                            );
                        }
                        Ok(error) => {
                            return Err(Report::from(error));
                        }
                        Err(report) => {
                            return Err(report);
                        }
                    },
                    Ok(_) => {
                        self.info.fp_brightness_percentage = Some(percentage);
                    }
                }
            }
            AppEvent::SetKeyboardBrightness(percentage) => {
                self.framework.set_kb_brightness(percentage);
                self.info.kb_brightness_percentage = Some(percentage);
            }
            AppEvent::SetTickInterval(interval_ms) => {
                self.config.set_tick_interval(interval_ms)?;
                event_loop.set_tick_interval(Duration::from_millis(interval_ms));
                self.tui.title.set_tick_interval(interval_ms);
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
