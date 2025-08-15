use crossterm::event::{self, Event, KeyCode};
use framework_lib::chromium_ec::CrosEc;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::*,
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
};
use std::time::Duration;
use tui_framework_experiment::toggle_switch::{State, ToggleSwitch};

use crate::framework::Framework;

const APP_TITLE: &str = " Framework System ";
const FOOTER_HELP: &str = "[Tab] Switch Focus  [Enter] Apply  [Esc] Cancel  [Q] Quit";
const NORMAL_CAPACITY_LOSS_MAX: f32 = 0.048;
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct App {
    framework: Framework,
    running: bool,
}

impl App {
    pub fn new() -> Self {
        let poll_interval = Duration::from_millis(1000);
        let ec = CrosEc::new();
        let framework = Framework::new(ec, poll_interval);

        App {
            framework,
            running: true,
        }
    }

    pub fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.running = false;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        self.framework.poll();

        while self.running {
            self.framework.poll_if_needed();

            terminal.draw(|frame| {
                self.render(frame);
            })?;

            let _ = self.handle_events();
        }

        Ok(())
    }

    pub fn render(&self, frame: &mut Frame) {
        let [title_area, main_area, footer_area] =
            Layout::vertical([Constraint::Max(3), Constraint::Min(0), Constraint::Max(3)])
                .areas(frame.area());

        self.render_title(frame, title_area);
        self.render_main(frame, main_area);
        self.render_footer(frame, footer_area);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(APP_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [
            smbios_version_area,
            charging_status_area,
            charge_percentage_area,
            max_charge_limit_area,
        ] = Layout::horizontal([
            Constraint::Max(6),
            Constraint::Max(13),
            Constraint::Max(4),
            Constraint::Max(9),
        ])
        .horizontal_margin(2)
        .spacing(2)
        .areas(block.inner(area));

        // BIOS version
        if let Some(smbios_version) = self.framework.controls.smbios_version() {
            frame.render_widget(
                Paragraph::new(format!("v{}", smbios_version)),
                smbios_version_area,
            );
        }

        let charge_percentage = self.framework.controls.charge_percentage();
        let charge_style = match charge_percentage {
            Some(charge_percentage) if charge_percentage < 15 => Style::new().red(),
            _ => Style::new().green(),
        };

        // Charging status
        frame.render_widget(
            Paragraph::new(self.framework.controls.charging_status()).style(charge_style),
            charging_status_area,
        );

        // Charge percentage
        if let Some(charge_percentage) = self.framework.controls.charge_percentage() {
            frame.render_widget(
                Paragraph::new(format!("{}%", charge_percentage)).style(charge_style),
                charge_percentage_area,
            );
        }

        // Max charge limit
        if let Some(max_charge_limit) = self.framework.controls.max_charge_limit() {
            frame.render_widget(
                Paragraph::new(format!("Max: {}%", max_charge_limit)),
                max_charge_limit_area,
            );
        }

        frame.render_widget(block, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
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

    fn render_main(&self, frame: &mut Frame, area: Rect) {
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Min(0)]).areas(area);
        let [charging_panel_area, privacy_and_smbios_panels_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Max(7)]).areas(left_area);
        let [privacy_panel_area, smbios_panel_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Min(0)])
                .areas(privacy_and_smbios_panels_area);
        let [brightness_panel_area] = Layout::vertical([Constraint::Min(0)]).areas(right_area);

        self.render_charge_panel(frame, charging_panel_area);

        self.render_privacy_panel(frame, privacy_panel_area);
        self.render_smbios_panel(frame, smbios_panel_area);

        self.render_brightness_panel(frame, brightness_panel_area);
    }

    fn render_charge_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Charge ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [
            charge_level_key_area,
            _empty1_key_area,
            charge_limit_key_area,
            _empty2_key_area,
            charger_voltage_key_area,
            charger_current_key_area,
            design_capacity_key_area,
            last_full_capacity_key_area,
            capacity_loss_key_area,
            cycle_count_key_area,
            capacity_loss_per_cycle_key_area,
        ] = Layout::vertical([
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
        ])
        .areas(keys_block.inner(keys_area));
        let [
            charge_level_value_area,
            _empty1_value_area,
            charge_limit_value_area,
            _empty2_value_area,
            charger_voltage_value_area,
            charger_current_value_area,
            design_capacity_value_area,
            last_full_capacity_value_area,
            capacity_loss_value_area,
            cycle_count_value_area,
            capacity_loss_per_cycle_value_area,
        ] = Layout::vertical([
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Max(1),
        ])
        .areas(values_block.inner(values_area));

        // Charge level
        self.render_charge_level(frame, charge_level_key_area, charge_level_value_area);

        // Max charge limit
        self.render_charge_limit(frame, charge_limit_key_area, charge_limit_value_area);

        // Charger voltage
        self.render_charger_voltage(frame, charger_voltage_key_area, charger_voltage_value_area);

        // Charger current
        self.render_charger_current(frame, charger_current_key_area, charger_current_value_area);

        // Design capacity
        self.render_design_capacity(frame, design_capacity_key_area, design_capacity_value_area);

        // Last full charge capacity
        self.render_last_full_charge_capacity(
            frame,
            last_full_capacity_key_area,
            last_full_capacity_value_area,
        );

        // Capacity loss
        self.render_capacity_loss(frame, capacity_loss_key_area, capacity_loss_value_area);

        // Cycle count
        self.render_cycle_count(frame, cycle_count_key_area, cycle_count_value_area);

        // Capacity loss per cycle
        self.render_capacity_loss_per_cycle(
            frame,
            capacity_loss_per_cycle_key_area,
            capacity_loss_per_cycle_value_area,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }

    fn render_charge_level(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let gauge = match self.framework.controls.charge_percentage() {
            Some(charge_percentage) => {
                let gauge_style = if charge_percentage < 15 {
                    Style::new().red().on_gray()
                } else {
                    Style::new().green().on_gray()
                };
                let label = format!(
                    "{} {}%",
                    self.framework.controls.charging_status(),
                    charge_percentage
                );

                Gauge::default()
                    .percent(charge_percentage as u16)
                    .label(label)
                    .gauge_style(gauge_style)
            }
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(Paragraph::new("Charge level"), key_area);
        frame.render_widget(gauge, value_area);
    }

    fn render_charge_limit(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let gauge = match self.framework.controls.max_charge_limit() {
            Some(max_charge_limit) => Gauge::default()
                .percent(max_charge_limit as u16)
                .gauge_style(Style::new().light_blue().on_gray()),
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(Paragraph::new("Max charge limit"), key_area);
        frame.render_widget(gauge, value_area);
    }

    fn render_charger_voltage(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let charger_voltage_text = match self.framework.controls.charger_voltage() {
            Some(charger_voltage) => format!("{} mV", charger_voltage),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger voltage"), key_area);
        frame.render_widget(Paragraph::new(charger_voltage_text), value_area);
    }

    fn render_charger_current(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let charger_current_text = match self.framework.controls.charger_current() {
            Some(charger_current) => format!("{} mA", charger_current),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger current"), key_area);
        frame.render_widget(Paragraph::new(charger_current_text), value_area);
    }

    fn render_design_capacity(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let design_capacity_text = match self.framework.controls.design_capacity() {
            Some(design_capacity) => format!("{} mAh", design_capacity),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Design capacity"), key_area);
        frame.render_widget(Paragraph::new(design_capacity_text), value_area);
    }

    fn render_last_full_charge_capacity(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
    ) {
        let last_full_charge_capacity_text =
            match self.framework.controls.last_full_charge_capacity() {
                Some(last_full_charge_capacity) => format!("{} mAh", last_full_charge_capacity),
                None => "N/A".to_string(),
            };

        frame.render_widget(Paragraph::new("Last full capacity"), key_area);
        frame.render_widget(Paragraph::new(last_full_charge_capacity_text), value_area);
    }

    fn render_capacity_loss(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let capacity_loss_text = match self.framework.controls.capacity_loss_percentage() {
            Some(capacity_loss_percentage) => {
                if capacity_loss_percentage > 0.0 {
                    format!("-{:.2}%", capacity_loss_percentage)
                } else {
                    format!("+{:.2}%", capacity_loss_percentage)
                }
            }
            _ => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Capacity loss"), key_area);
        frame.render_widget(Paragraph::new(capacity_loss_text), value_area);
    }

    fn render_cycle_count(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let cycle_count_text = match self.framework.controls.cycle_count() {
            Some(cycle_count) => format!("{}", cycle_count),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Cycle count"), key_area);
        frame.render_widget(Paragraph::new(cycle_count_text), value_area);
    }

    fn render_capacity_loss_per_cycle(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let capacity_loss_per_cycle = self.framework.controls.capacity_loss_per_cycle();

        let capacity_loss_per_cycle_style = match capacity_loss_per_cycle {
            Some(capacity_loss_per_cycle) => {
                if capacity_loss_per_cycle < NORMAL_CAPACITY_LOSS_MAX {
                    Style::new().green()
                } else {
                    Style::new().red()
                }
            }
            None => Style::default(),
        };
        let capacity_loss_per_cycle_text = match capacity_loss_per_cycle {
            Some(capacity_loss_per_cycle) if capacity_loss_per_cycle > NORMAL_CAPACITY_LOSS_MAX => {
                format!(
                    "{:.3}% (normal loss is 0.025-0.048%)",
                    capacity_loss_per_cycle
                )
            }
            Some(capacity_loss_per_cycle) => format!("{:.3}%", capacity_loss_per_cycle),
            _ => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Capacity loss per cycle"), key_area);
        frame.render_widget(
            Paragraph::new(capacity_loss_per_cycle_text).style(capacity_loss_per_cycle_style),
            value_area,
        );
    }

    fn render_privacy_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Privacy ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [mic_area, camera_area] = Layout::vertical([Constraint::Max(1), Constraint::Max(1)])
            .spacing(1)
            .horizontal_margin(2)
            .vertical_margin(1)
            .areas(block.inner(area));

        // Micrhophone
        self.render_mic(frame, mic_area);

        // Camera
        self.render_camera(frame, camera_area);

        // Render block
        frame.render_widget(block, area);
    }

    fn render_mic(&self, frame: &mut Frame, area: Rect) {
        let toggle_state = if self.framework.controls.is_microphone_enabled() {
            State::On
        } else {
            State::Off
        };
        let toggle = &ToggleSwitch::new("Microphone", toggle_state);

        frame.render_widget(toggle, area);
    }

    fn render_camera(&self, frame: &mut Frame, area: Rect) {
        let toggle_state = if self.framework.controls.is_camera_enabled() {
            State::On
        } else {
            State::Off
        };
        let toggle = &ToggleSwitch::new("Camera", toggle_state);

        frame.render_widget(toggle, area);
    }

    fn render_brightness_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Brightness ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [fp_brightness_key_area, kb_brightness_key_area] =
            Layout::vertical([Constraint::Max(1), Constraint::Max(1)])
                .spacing(1)
                .areas(keys_block.inner(keys_area));
        let [fp_brightness_value_area, kb_brightness_value_area] =
            Layout::vertical([Constraint::Max(1), Constraint::Max(1)])
                .spacing(1)
                .areas(values_block.inner(values_area));

        // Fingerprint brightness
        self.render_fp_brightness(frame, fp_brightness_key_area, fp_brightness_value_area);

        // Keyboard brightness
        self.render_kb_brightness(frame, kb_brightness_key_area, kb_brightness_value_area);

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }

    fn render_fp_brightness(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let gauge = match self.framework.controls.fp_brightness_percentage() {
            Some(fp_brightness_percentage) => Gauge::default()
                .percent(fp_brightness_percentage as u16)
                .gauge_style(Style::new().yellow()),
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(Paragraph::new("Fingerprint LED brigtness"), key_area);

        match self.framework.controls.fp_brightness_level() {
            Some(fp_brightness_level) => {
                let [gauge_area, level_area] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                        .areas(value_area);

                let fp_brightness_level_text = format!("{:?}", fp_brightness_level);

                frame.render_widget(gauge, gauge_area);
                frame.render_widget(Paragraph::new(fp_brightness_level_text), level_area);
            }
            None => {
                frame.render_widget(gauge, value_area);
            }
        }
    }

    fn render_kb_brightness(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let gauge = match self.framework.controls.kb_brightness_percentage() {
            Some(kb_brightness_percentage) => Gauge::default()
                .percent(kb_brightness_percentage as u16)
                .gauge_style(Style::new().yellow()),
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(Paragraph::new("Keyboard brightness"), key_area);
        frame.render_widget(gauge, value_area);
    }

    fn render_smbios_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" BIOS ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [
            smbios_vendor_key_area,
            smbios_version_key_area,
            smbios_release_date_key_area,
        ] = Layout::vertical([Constraint::Max(1), Constraint::Max(1), Constraint::Max(1)])
            .areas(keys_block.inner(keys_area));
        let [
            smbios_vendor_value_area,
            smbios_version_value_area,
            smbios_release_date_value_area,
        ] = Layout::vertical([Constraint::Max(1), Constraint::Max(1), Constraint::Max(1)])
            .areas(values_block.inner(values_area));

        // Vendor
        self.render_smbios_vendor(frame, smbios_vendor_key_area, smbios_vendor_value_area);

        // Version
        self.render_smbios_version(frame, smbios_version_key_area, smbios_version_value_area);

        // Release date
        self.render_smbios_release_date(
            frame,
            smbios_release_date_key_area,
            smbios_release_date_value_area,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }

    fn render_smbios_version(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let smbios_version_text = match self.framework.controls.smbios_version() {
            Some(smbios_version) => smbios_version,
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Version"), key_area);
        frame.render_widget(Paragraph::new(smbios_version_text), value_area);
    }

    fn render_smbios_release_date(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let smbios_release_date_text = match self.framework.controls.smbios_release_date() {
            Some(smbios_release_date) => smbios_release_date,
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Release date"), key_area);
        frame.render_widget(Paragraph::new(smbios_release_date_text), value_area);
    }

    fn render_smbios_vendor(&self, frame: &mut Frame, key_area: Rect, value_area: Rect) {
        let smbios_vendor_text = match self.framework.controls.smbios_vendor() {
            Some(smbios_vendor) => smbios_vendor,
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Vendor"), key_area);
        frame.render_widget(Paragraph::new(smbios_vendor_text), value_area);
    }
}
