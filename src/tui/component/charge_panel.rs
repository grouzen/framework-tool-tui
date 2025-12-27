use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::Styled,
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};

use crate::{
    app::AppEvent,
    framework::info::FrameworkInfo,
    tui::{
        component::{AdjustableComponent, AdjustablePanel, Component},
        control::percentage_control,
        theme::Theme,
    },
};

const NORMAL_CAPACITY_LOSS_MAX: f32 = 0.048;
const MAX_CHARGE_LIMIT_CONTROL_INDEX: usize = 0;

pub struct ChargePanelComponent(AdjustablePanel);

impl Default for ChargePanelComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargePanelComponent {
    pub fn new() -> Self {
        Self(AdjustablePanel {
            selected: false,
            controls: vec![percentage_control(0)],
            selected_control: MAX_CHARGE_LIMIT_CONTROL_INDEX,
        })
    }

    fn render_charge_level(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let gauge = match info.charge_percentage {
            Some(charge_percentage) => {
                let gauge_style = if charge_percentage < 15 {
                    Style::default()
                        .fg(theme.indication_warning)
                        .bg(theme.bar_background)
                } else {
                    Style::default()
                        .fg(theme.indication_ok)
                        .bg(theme.bar_background)
                };
                let label = format!("{} {}%", info.charging_status, charge_percentage);

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

    fn render_max_charge_limit(
        &mut self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let style = self.0.adjustable_control_style(
            Style::new().fg(theme.background).bg(theme.text),
            Style::default(),
            MAX_CHARGE_LIMIT_CONTROL_INDEX,
        );

        let max_charge_limit = if self
            .0
            .is_panel_selected_and_control_focused_by_index(MAX_CHARGE_LIMIT_CONTROL_INDEX)
        {
            self.0.get_selected_control().get_percentage_value()
        } else if let Some(value) = info.max_charge_limit {
            self.0.set_percentage_control_by_index(
                MAX_CHARGE_LIMIT_CONTROL_INDEX,
                percentage_control(value),
            );

            Some(value)
        } else {
            None
        };

        let gauge = match max_charge_limit {
            Some(max_charge_limit) => {
                let style = self.0.adjustable_control_style(
                    Style::new().gray().on_black(),
                    Style::default()
                        .fg(theme.charge_bar)
                        .bg(theme.bar_background),
                    MAX_CHARGE_LIMIT_CONTROL_INDEX,
                );
                let label = if self
                    .0
                    .is_panel_selected_and_control_focused_by_index(MAX_CHARGE_LIMIT_CONTROL_INDEX)
                {
                    format!("◀ {:3}% ▶", max_charge_limit)
                } else {
                    format!("{:3}%", max_charge_limit)
                };

                Gauge::default()
                    .percent(max_charge_limit as u16)
                    .label(label)
                    .gauge_style(style)
            }
            None => Gauge::default().percent(0).label("N/A").gauge_style(style),
        };

        frame.render_widget(
            Paragraph::new("Max charge limit").set_style(style),
            key_area,
        );
        frame.render_widget(gauge, value_area);
    }

    fn render_charger_voltage(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let charger_voltage_text = match info.charger_voltage {
            Some(charger_voltage) => format!("{} mV", charger_voltage),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger voltage"), key_area);
        frame.render_widget(
            Paragraph::new(charger_voltage_text).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_charger_current(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let charger_current_text = match info.charger_current {
            Some(charger_current) => format!("{} mA", charger_current),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Charger current"), key_area);
        frame.render_widget(
            Paragraph::new(charger_current_text).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_design_capacity(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let design_capacity_text = match info.design_capacity {
            Some(design_capacity) => format!("{} mAh", design_capacity),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Design capacity"), key_area);
        frame.render_widget(
            Paragraph::new(design_capacity_text).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_last_full_charge_capacity(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let last_full_charge_capacity_text = match info.last_full_charge_capacity {
            Some(last_full_charge_capacity) => format!("{} mAh", last_full_charge_capacity),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Last full capacity"), key_area);
        frame.render_widget(
            Paragraph::new(last_full_charge_capacity_text)
                .style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_capacity_loss(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let capacity_loss_text = match info.capacity_loss_percentage {
            Some(capacity_loss_percentage) => {
                let inverted = -capacity_loss_percentage;

                format!("{:+.2}%", inverted)
            }
            _ => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Capacity loss"), key_area);
        frame.render_widget(
            Paragraph::new(capacity_loss_text).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_cycle_count(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let cycle_count_text = match info.cycle_count {
            Some(cycle_count) => format!("{}", cycle_count),
            None => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Cycle count"), key_area);
        frame.render_widget(
            Paragraph::new(cycle_count_text).style(Style::default().fg(theme.informative_text)),
            value_area,
        );
    }

    fn render_capacity_loss_per_cycle(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let capacity_loss_per_cycle = info.capacity_loss_per_cycle;

        let capacity_loss_per_cycle_style = match capacity_loss_per_cycle {
            Some(capacity_loss_per_cycle) => {
                if capacity_loss_per_cycle < NORMAL_CAPACITY_LOSS_MAX {
                    Style::default().fg(theme.indication_ok)
                } else {
                    Style::default().fg(theme.indication_warning)
                }
            }
            None => Style::default(),
        };
        let capacity_loss_per_cycle_text = match capacity_loss_per_cycle {
            Some(capacity_loss_per_cycle) => {
                let inverted = -capacity_loss_per_cycle;

                if capacity_loss_per_cycle > NORMAL_CAPACITY_LOSS_MAX {
                    format!("{:+.3}% (expected 0.025-0.048%)", inverted)
                } else {
                    format!("{:+.3}%", inverted)
                }
            }
            _ => "N/A".to_string(),
        };

        frame.render_widget(Paragraph::new("Capacity loss per cycle"), key_area);
        frame.render_widget(
            Paragraph::new(capacity_loss_per_cycle_text).style(capacity_loss_per_cycle_style),
            value_area,
        );
    }
}

impl AdjustableComponent for ChargePanelComponent {
    fn panel(&mut self) -> &mut AdjustablePanel {
        &mut self.0
    }
}

impl Component for ChargePanelComponent {
    fn handle_input(&mut self, event: Event) -> Option<crate::app::AppEvent> {
        let mut app_event = None;

        if self.0.is_selected() {
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Down => self.0.cycle_controls_down(),
                    KeyCode::Up => self.0.cycle_controls_up(),
                    KeyCode::Enter => {
                        match self.0.get_selected_and_focused_control() {
                            Some(control)
                                if self.0.selected_control == MAX_CHARGE_LIMIT_CONTROL_INDEX =>
                            {
                                if let Some(value) = control.get_percentage_value() {
                                    app_event = Some(AppEvent::SetMaxChargeLimit(value));
                                }
                            }
                            _ => {}
                        }

                        self.0.toggle_selected_control_focus()
                    }
                    KeyCode::Left => self.0.adjust_focused_percentage_control_by_delta(-5),
                    KeyCode::Right => self.0.adjust_focused_percentage_control_by_delta(5),
                    KeyCode::Esc => self.0.toggle_selected_control_focus(),
                    _ => {}
                }
            }
        }

        app_event
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .title(" Charge ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.0.borders_style(theme));

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [charge_level_key_area, _empty1_key_area, charge_limit_key_area, _empty2_key_area, charger_voltage_key_area, charger_current_key_area, design_capacity_key_area, last_full_capacity_key_area, capacity_loss_key_area, cycle_count_key_area, capacity_loss_per_cycle_key_area] =
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .areas(keys_block.inner(keys_area));
        let [charge_level_value_area, _empty1_value_area, charge_limit_value_area, _empty2_value_area, charger_voltage_value_area, charger_current_value_area, design_capacity_value_area, last_full_capacity_value_area, capacity_loss_value_area, cycle_count_value_area, capacity_loss_per_cycle_value_area] =
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .horizontal_margin(1)
            .areas(values_block.inner(values_area));

        // Charge level
        self.render_charge_level(
            frame,
            charge_level_key_area,
            charge_level_value_area,
            theme,
            info,
        );

        // Max charge limit
        self.render_max_charge_limit(
            frame,
            charge_limit_key_area,
            charge_limit_value_area,
            theme,
            info,
        );

        // Charger voltage
        self.render_charger_voltage(
            frame,
            charger_voltage_key_area,
            charger_voltage_value_area,
            theme,
            info,
        );

        // Charger current
        self.render_charger_current(
            frame,
            charger_current_key_area,
            charger_current_value_area,
            theme,
            info,
        );

        // Design capacity
        self.render_design_capacity(
            frame,
            design_capacity_key_area,
            design_capacity_value_area,
            theme,
            info,
        );

        // Last full charge capacity
        self.render_last_full_charge_capacity(
            frame,
            last_full_capacity_key_area,
            last_full_capacity_value_area,
            theme,
            info,
        );

        // Capacity loss
        self.render_capacity_loss(
            frame,
            capacity_loss_key_area,
            capacity_loss_value_area,
            theme,
            info,
        );

        // Cycle count
        self.render_cycle_count(
            frame,
            cycle_count_key_area,
            cycle_count_value_area,
            theme,
            info,
        );

        // Capacity loss per cycle
        self.render_capacity_loss_per_cycle(
            frame,
            capacity_loss_per_cycle_key_area,
            capacity_loss_per_cycle_value_area,
            theme,
            info,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

    use crate::tui::{
        component::{
            charge_panel::{ChargePanelComponent, MAX_CHARGE_LIMIT_CONTROL_INDEX},
            Component,
        },
        control::AdjustableControl,
    };

    #[test]
    fn handle_input_enter_when_panel_selected() {
        let mut panel = ChargePanelComponent::new();
        let event = Event::Key(KeyEvent::from(KeyCode::Enter));

        panel.0.toggle();
        let _ = panel.handle_input(event);

        assert!(panel.0.is_selected());
        assert!(panel.0.controls.len() == 1);
        assert!(panel.0.controls[MAX_CHARGE_LIMIT_CONTROL_INDEX].is_focused())
    }

    #[test]
    fn handle_input_left_for_focused_percentage_control_stay_in_range() {
        let mut panel = ChargePanelComponent::new();
        let event = Event::Key(KeyEvent::from(KeyCode::Left));

        panel.0.toggle();
        panel.0.toggle_selected_control_focus();
        let _ = panel.handle_input(event);

        assert!(panel.0.is_selected());
        assert!(panel.0.controls.len() == 1);
        assert!(panel
            .0
            .is_panel_selected_and_control_focused_by_index(MAX_CHARGE_LIMIT_CONTROL_INDEX));
        assert!(matches!(
            panel.0.get_selected_control(),
            AdjustableControl::Percentage(true, 0)
        ));
    }
}
