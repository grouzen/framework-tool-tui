use std::sync::Arc;

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
    framework::{
        fingerprint::{led_brightness_percentage_to_level_name, Fingerprint},
        info::FrameworkInfo,
    },
    tui::{
        component::{AdjustableComponent, AdjustablePanel, Component},
        control::{percentage_control, AdjustableControl},
        theme::Theme,
    },
};

const FINGERPRINT_BRIGHTNESS_CONTROL_INDEX: usize = 0;
const KEYBOARD_BRIGHTNESS_CONTROL_INDEX: usize = 1;

pub struct BrightnessPanelComponent {
    panel: AdjustablePanel,
    fingerprint: Arc<Fingerprint>,
}

impl BrightnessPanelComponent {
    pub fn new(fingerprint: Arc<Fingerprint>) -> Self {
        let panel = AdjustablePanel {
            selected: false,
            controls: vec![percentage_control(0), percentage_control(0)],
            selected_control: FINGERPRINT_BRIGHTNESS_CONTROL_INDEX,
        };

        Self { panel, fingerprint }
    }

    fn render_fp_brightness(
        &mut self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let style = self.panel.adjustable_control_style(
            Style::new().fg(theme.background).bg(theme.text),
            Style::default(),
            FINGERPRINT_BRIGHTNESS_CONTROL_INDEX,
        );

        let fp_brightness_percentage = if self
            .panel
            .is_panel_selected_and_control_focused_by_index(FINGERPRINT_BRIGHTNESS_CONTROL_INDEX)
        {
            self.panel.get_selected_control().get_percentage_value()
        } else if let Some(value) = info.fp_brightness_percentage {
            self.panel.set_percentage_control_by_index(
                FINGERPRINT_BRIGHTNESS_CONTROL_INDEX,
                percentage_control(value),
            );

            Some(value)
        } else {
            None
        };

        let gauge = match fp_brightness_percentage {
            Some(fp_brightness_percentage) => {
                let style = self.panel.adjustable_control_style(
                    Style::new().gray().on_black(),
                    Style::default().fg(theme.brightness_bar),
                    FINGERPRINT_BRIGHTNESS_CONTROL_INDEX,
                );
                let level_name = led_brightness_percentage_to_level_name(fp_brightness_percentage);
                let label = if self.panel.is_panel_selected_and_control_focused_by_index(
                    FINGERPRINT_BRIGHTNESS_CONTROL_INDEX,
                ) {
                    format!("◀ {} {:3}% ▶", level_name, fp_brightness_percentage)
                } else {
                    format!("{} {:3}%", level_name, fp_brightness_percentage)
                };

                Gauge::default()
                    .percent(fp_brightness_percentage as u16)
                    .label(label)
                    .gauge_style(style)
            }
            None => Gauge::default().percent(0).label("N/A").gauge_style(style),
        };

        frame.render_widget(
            Paragraph::new("Fingerprint brightness").set_style(style),
            key_area,
        );
        frame.render_widget(gauge, value_area);
    }

    fn render_kb_brightness(
        &mut self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        theme: &Theme,
        info: &FrameworkInfo,
    ) {
        let style = self.panel.adjustable_control_style(
            Style::new().fg(theme.background).bg(theme.text),
            Style::default(),
            KEYBOARD_BRIGHTNESS_CONTROL_INDEX,
        );

        let kb_brightness_percentage = if self
            .panel
            .is_panel_selected_and_control_focused_by_index(KEYBOARD_BRIGHTNESS_CONTROL_INDEX)
        {
            self.panel.get_selected_control().get_percentage_value()
        } else if let Some(value) = info.kb_brightness_percentage {
            self.panel.set_percentage_control_by_index(
                KEYBOARD_BRIGHTNESS_CONTROL_INDEX,
                percentage_control(value),
            );

            Some(value)
        } else {
            None
        };

        let gauge = match kb_brightness_percentage {
            Some(kb_brightness_percentage) => {
                let style = self.panel.adjustable_control_style(
                    Style::new().gray().on_black(),
                    Style::default().fg(theme.brightness_bar),
                    KEYBOARD_BRIGHTNESS_CONTROL_INDEX,
                );
                let label = if self.panel.is_panel_selected_and_control_focused_by_index(
                    KEYBOARD_BRIGHTNESS_CONTROL_INDEX,
                ) {
                    format!("◀ {:3}% ▶", kb_brightness_percentage)
                } else {
                    format!("{:3}%", kb_brightness_percentage)
                };

                Gauge::default()
                    .percent(kb_brightness_percentage as u16)
                    .label(label)
                    .gauge_style(style)
            }
            None => Gauge::default().percent(0).label("N/A").gauge_style(style),
        };

        frame.render_widget(
            Paragraph::new("Keyboard brightness").set_style(style),
            key_area,
        );
        frame.render_widget(gauge, value_area);
    }

    fn adjust_focused_fp_brightness_control(&mut self, delta: i8) {
        if let Some(AdjustableControl::Percentage(focused, value)) =
            self.panel.get_selected_and_focused_control()
        {
            let new_value = self
                .fingerprint
                .adjust_led_brightness_by_delta(*value, delta);

            if (0..=100).contains(&new_value) {
                self.panel.controls[self.panel.selected_control] =
                    AdjustableControl::Percentage(*focused, new_value);
            }
        }
    }
}

impl AdjustableComponent for BrightnessPanelComponent {
    fn panel(&mut self) -> &mut AdjustablePanel {
        &mut self.panel
    }
}

impl Component for BrightnessPanelComponent {
    fn handle_input(&mut self, event: Event) -> Option<crate::app::AppEvent> {
        let mut app_event = None;

        if self.panel.is_selected() {
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Down => self.panel.cycle_controls_down(),
                    KeyCode::Up => self.panel.cycle_controls_up(),
                    KeyCode::Enter => {
                        match self.panel.get_selected_and_focused_control() {
                            Some(control)
                                if self.panel.selected_control
                                    == FINGERPRINT_BRIGHTNESS_CONTROL_INDEX =>
                            {
                                if let Some(value) = control.get_percentage_value() {
                                    app_event = Some(AppEvent::SetFingerprintBrightness(value));
                                }
                            }
                            Some(control)
                                if self.panel.selected_control
                                    == KEYBOARD_BRIGHTNESS_CONTROL_INDEX =>
                            {
                                if let Some(value) = control.get_percentage_value() {
                                    app_event = Some(AppEvent::SetKeyboardBrightness(value));
                                }
                            }
                            _ => {}
                        }

                        self.panel.toggle_selected_control_focus()
                    }
                    KeyCode::Left => {
                        if self.panel.selected_control == KEYBOARD_BRIGHTNESS_CONTROL_INDEX {
                            self.panel.adjust_focused_percentage_control_by_delta(-5)
                        } else {
                            self.adjust_focused_fp_brightness_control(-5);
                        }
                    }
                    KeyCode::Right => {
                        if self.panel.selected_control == KEYBOARD_BRIGHTNESS_CONTROL_INDEX {
                            self.panel.adjust_focused_percentage_control_by_delta(5)
                        } else {
                            self.adjust_focused_fp_brightness_control(5);
                        }
                    }
                    KeyCode::Esc => self.panel.toggle_selected_control_focus(),
                    _ => {}
                }
            }
        }

        app_event
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .title(" Brightness ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.panel.borders_style(theme));

        let [keys_area, values_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(block.inner(area));

        let keys_block = Block::default().borders(Borders::NONE);
        let values_block = Block::default().borders(Borders::NONE);

        let [fp_brightness_key_area, kb_brightness_key_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .spacing(1)
                .areas(keys_block.inner(keys_area));
        let [fp_brightness_value_area, kb_brightness_value_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .spacing(1)
                .horizontal_margin(1)
                .areas(values_block.inner(values_area));

        // Fingerprint brightness
        self.render_fp_brightness(
            frame,
            fp_brightness_key_area,
            fp_brightness_value_area,
            theme,
            info,
        );

        // Keyboard brightness
        self.render_kb_brightness(
            frame,
            kb_brightness_key_area,
            kb_brightness_value_area,
            theme,
            info,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }
}
