use color_eyre::eyre::Report;
use framework_lib::chromium_ec::{CrosEc, EcError, EcResponseStatus};
use ratatui::{prelude::Backend, Terminal};
use std::{sync::Arc, time::Duration};

use crate::{
    event::{Event, EventLoop},
    framework::{fingerprint::Fingerprint, info::FrameworkInfo, EcErrorWrapper, Framework},
    tui::Tui,
};

pub const APP_TITLE: &str = " Framework System ";
pub const FOOTER_HELP: &str = "[Tab] Switch panels [Up/Down] Scroll [Enter] Edit/Apply [Left/Right] Adjust value [b/n] Switch theme [Esc] Cancel [q] Quit";
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

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let ec = CrosEc::new();
        let fingerprint = Arc::new(Fingerprint::new(&ec)?);
        // Pre-fetch framework info
        let mut framework = Framework::new(ec, fingerprint.clone());
        let info = framework.get_info();
        let tui = Tui::new(fingerprint, &info);

        Ok(Self {
            framework,
            info,
            running: true,
            tui,
        })
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        let mut event_loop = EventLoop::new();

        event_loop.run(Duration::from_millis(1000));

        while self.running {
            self.tui.render(terminal, &self.info)?;

            match event_loop.next().await? {
                Event::Tick => {
                    self.info = self.framework.get_info();
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
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
