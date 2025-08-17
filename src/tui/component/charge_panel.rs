use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
};

use crate::{
    framework::FrameworkControls,
    tui::component::{Component, SelectableComponent},
};

const NORMAL_CAPACITY_LOSS_MAX: f32 = 0.048;

pub struct ChargePanelComponent {
    selected: bool,
}

impl Default for ChargePanelComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargePanelComponent {
    pub fn new() -> Self {
        Self { selected: false }
    }

    fn borders_style(&self) -> Style {
        if self.selected {
            Style::new().yellow().bold()
        } else {
            Style::default()
        }
    }

    fn render_charge_level(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let gauge = match controls.charge_percentage() {
            Some(charge_percentage) => {
                let gauge_style = if charge_percentage < 15 {
                    Style::new().red().on_gray()
                } else {
                    Style::new().green().on_gray()
                };
                let label = format!("{} {}%", controls.charging_status(), charge_percentage);

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

    fn render_charge_limit(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let gauge = match controls.max_charge_limit() {
            Some(max_charge_limit) => Gauge::default()
                .percent(max_charge_limit as u16)
                .gauge_style(Style::new().light_blue().on_gray()),
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(Paragraph::new("Max charge limit"), key_area);
        frame.render_widget(gauge, value_area);
    }

    fn render_charger_voltage(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let charger_voltage_text = match controls.charger_voltage() {
            Some(charger_voltage) => format!("{} mV", charger_voltage),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger voltage"), key_area);
        frame.render_widget(Paragraph::new(charger_voltage_text), value_area);
    }

    fn render_charger_current(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let charger_current_text = match controls.charger_current() {
            Some(charger_current) => format!("{} mA", charger_current),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger current"), key_area);
        frame.render_widget(Paragraph::new(charger_current_text), value_area);
    }

    fn render_design_capacity(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let design_capacity_text = match controls.design_capacity() {
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
        controls: &FrameworkControls,
    ) {
        let last_full_charge_capacity_text = match controls.last_full_charge_capacity() {
            Some(last_full_charge_capacity) => format!("{} mAh", last_full_charge_capacity),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Last full capacity"), key_area);
        frame.render_widget(Paragraph::new(last_full_charge_capacity_text), value_area);
    }

    fn render_capacity_loss(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let capacity_loss_text = match controls.capacity_loss_percentage() {
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

    fn render_cycle_count(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let cycle_count_text = match controls.cycle_count() {
            Some(cycle_count) => format!("{}", cycle_count),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Cycle count"), key_area);
        frame.render_widget(Paragraph::new(cycle_count_text), value_area);
    }

    fn render_capacity_loss_per_cycle(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let capacity_loss_per_cycle = controls.capacity_loss_per_cycle();

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
}

impl SelectableComponent for ChargePanelComponent {
    fn toggle(&mut self) {
        self.selected = !self.selected;
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Component for ChargePanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let block = Block::default()
            .title(" Charge ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.borders_style());

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
        self.render_charge_level(
            frame,
            charge_level_key_area,
            charge_level_value_area,
            controls,
        );

        // Max charge limit
        self.render_charge_limit(
            frame,
            charge_limit_key_area,
            charge_limit_value_area,
            controls,
        );

        // Charger voltage
        self.render_charger_voltage(
            frame,
            charger_voltage_key_area,
            charger_voltage_value_area,
            controls,
        );

        // Charger current
        self.render_charger_current(
            frame,
            charger_current_key_area,
            charger_current_value_area,
            controls,
        );

        // Design capacity
        self.render_design_capacity(
            frame,
            design_capacity_key_area,
            design_capacity_value_area,
            controls,
        );

        // Last full charge capacity
        self.render_last_full_charge_capacity(
            frame,
            last_full_capacity_key_area,
            last_full_capacity_value_area,
            controls,
        );

        // Capacity loss
        self.render_capacity_loss(
            frame,
            capacity_loss_key_area,
            capacity_loss_value_area,
            controls,
        );

        // Cycle count
        self.render_cycle_count(
            frame,
            cycle_count_key_area,
            cycle_count_value_area,
            controls,
        );

        // Capacity loss per cycle
        self.render_capacity_loss_per_cycle(
            frame,
            capacity_loss_per_cycle_key_area,
            capacity_loss_per_cycle_value_area,
            controls,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }
}
