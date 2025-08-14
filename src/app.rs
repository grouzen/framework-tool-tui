use crossterm::event::{self, Event, KeyCode};
use framework_lib::chromium_ec::CrosEc;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};
use std::time::Duration;

use crate::framework::Framework;

const APP_TITLE: &str = "FRAMEWORK SYSTEM TUI";
const FOOTER_HELP: &str = "[Tab] Switch Focus  [Enter] Apply  [Esc] Cancel  [Q] Quit";
const NORMAL_CAPACITY_LOSS_MAX: f32 = 0.048;

pub struct App {
    pub framework: Framework,
    pub running: bool,
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
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.running = false;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn run(
        &mut self,
        terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    ) -> anyhow::Result<()> {
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
        let block = Block::default().title(APP_TITLE).borders(Borders::ALL);

        frame.render_widget(block, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().title("HELP").borders(Borders::ALL);

        frame.render_widget(Paragraph::new(FOOTER_HELP).block(block), area);
    }

    fn render_main(&self, frame: &mut Frame, area: Rect) {
        let [main_left_area] = Layout::vertical([Constraint::Min(0)]).areas(area);
        let [charging_panel_area] = Layout::horizontal([Constraint::Min(0)]).areas(main_left_area);

        self.render_charge_panel(frame, charging_panel_area);
    }

    fn render_charge_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().title("CHARGE").borders(Borders::ALL);

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [
            charge_level_key_area,
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
        ])
        .areas(keys_block.inner(keys_area));
        let [
            charge_level_value_area,
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
        ])
        .areas(values_block.inner(values_area));

        // Charge level
        frame.render_widget(Paragraph::new("Charge level"), charge_level_key_area);
        self.render_charge_level_value(frame, charge_level_value_area);

        // Charger voltage
        let charger_voltage_text = match self.framework.controls.charger_voltage() {
            Some(voltage) => format!("{} mV", voltage),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger voltage"), charger_voltage_key_area);
        frame.render_widget(
            Paragraph::new(charger_voltage_text),
            charger_voltage_value_area,
        );

        // Charger current
        let charger_current_text = match self.framework.controls.charger_current() {
            Some(current) => format!("{} mA", current),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger current"), charger_current_key_area);
        frame.render_widget(
            Paragraph::new(charger_current_text),
            charger_current_value_area,
        );

        // Design capacity
        let design_capacity_text = match self.framework.controls.design_capacity() {
            Some(capacity) => format!("{} mAh", capacity),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Design capacity"), design_capacity_key_area);
        frame.render_widget(
            Paragraph::new(design_capacity_text),
            design_capacity_value_area,
        );

        // Last full charge capacity
        let last_full_charge_capacity_text =
            match self.framework.controls.last_full_charge_capacity() {
                Some(capacity) => format!("{} mAh", capacity),
                None => "N/A".to_string(),
            };

        frame.render_widget(
            Paragraph::new("Last full capacity"),
            last_full_capacity_key_area,
        );
        frame.render_widget(
            Paragraph::new(last_full_charge_capacity_text),
            last_full_capacity_value_area,
        );

        // Capacity loss
        let capacity_loss_percentage = match (
            self.framework.controls.design_capacity(),
            self.framework.controls.last_full_charge_capacity(),
        ) {
            (Some(design), Some(last)) => {
                Some(((design as f32 - last as f32) / design as f32) * 100.0)
            }
            _ => None,
        };
        let capacity_loss_text = match capacity_loss_percentage {
            Some(loss_percentage) => {
                if loss_percentage > 0.0 {
                    format!("-{:.2}%", loss_percentage)
                } else {
                    format!("+{:.2}%", loss_percentage)
                }
            }
            _ => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Capacity loss"), capacity_loss_key_area);
        frame.render_widget(Paragraph::new(capacity_loss_text), capacity_loss_value_area);

        // Cycle count
        let cycle_count_text = match self.framework.controls.cycle_count() {
            Some(count) => format!("{}", count),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Cycle count"), cycle_count_key_area);
        frame.render_widget(Paragraph::new(cycle_count_text), cycle_count_value_area);

        // Capacity loss per cycle
        let capacity_loss_per_cycle = match (
            capacity_loss_percentage,
            self.framework.controls.cycle_count(),
        ) {
            (Some(loss_percentage), Some(cycle_count)) => {
                Some(loss_percentage / (cycle_count as f32))
            }
            _ => None,
        };

        let capacity_loss_per_cycle_style = match capacity_loss_per_cycle {
            Some(loss_per_cycle) => {
                if loss_per_cycle < NORMAL_CAPACITY_LOSS_MAX {
                    Style::new().green()
                } else {
                    Style::new().red()
                }
            }
            None => Style::default(),
        };
        let capacity_loss_per_cycle_text = match capacity_loss_per_cycle {
            Some(loss_per_cycle) if loss_per_cycle > NORMAL_CAPACITY_LOSS_MAX => {
                format!("{:.3}% (normal loss is 0.025-0.048%)", loss_per_cycle)
            }
            Some(loss_per_cycle) => format!("{:.3}%", loss_per_cycle),
            _ => "N/A".to_string(),
        };

        frame.render_widget(
            Paragraph::new("Capacity loss per cycle"),
            capacity_loss_per_cycle_key_area,
        );
        frame.render_widget(
            Paragraph::new(capacity_loss_per_cycle_text).style(capacity_loss_per_cycle_style),
            capacity_loss_per_cycle_value_area,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }

    fn render_charge_level_value(&self, frame: &mut Frame, area: Rect) {
        let [label_area, gauge_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(area);

        let is_charging = self.framework.controls.is_charging();
        let is_ac_connected = self.framework.controls.is_ac_connected();

        let label = Paragraph::new(match (is_charging, is_ac_connected) {
            (true, true) => "Charging",
            (false, true) => "Fully charged",
            (false, false) => "Discharging",
            (true, false) => "???",
        });
        let gauge = match self.framework.controls.charge_percentage() {
            Some(charge_level) => {
                let gauge_style = if charge_level < 15 {
                    Style::new().red().on_gray()
                } else {
                    Style::new().green().on_gray()
                };

                Gauge::default()
                    .percent(charge_level as u16)
                    .gauge_style(gauge_style)
            }
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(label, label_area);
        frame.render_widget(gauge, gauge_area);
    }
}
