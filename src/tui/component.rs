pub mod pd_ports_panel;
use ratatui::{crossterm::event::Event, layout::Rect, prelude::*, Frame};

use crate::{
    app::AppEvent,
    framework::info::FrameworkInfo,
    tui::{control::AdjustableControl, theme::Theme},
};

pub mod brightness_panel;
pub mod charge_graph_panel;
pub mod charge_panel;
pub mod charge_panels;
pub mod footer;
pub mod main;
pub mod privacy_panel;
pub mod smbios_panel;
pub mod title;

pub trait Component {
    fn handle_input(&mut self, _event: Event) -> Option<AppEvent> {
        None
    }

    fn render(&mut self, _frame: &mut Frame, _area: Rect, _theme: &Theme, _info: &FrameworkInfo) {}
}

pub trait AdjustableComponent: Component {
    fn panel(&mut self) -> &mut AdjustablePanel;
}

pub struct AdjustablePanel {
    selected: bool,
    controls: Vec<AdjustableControl>,
    selected_control: usize,
}

impl AdjustablePanel {
    fn toggle(&mut self) {
        self.selected = !self.is_selected();
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn cycle_controls_up(&mut self) {
        let len = self.controls.len();

        if self.selected_control == 0 {
            self.selected_control = len - 1;
        } else {
            self.selected_control -= 1;
        }
    }

    fn cycle_controls_down(&mut self) {
        let len = self.controls.len();

        if self.selected_control < len - 1 {
            self.selected_control += 1;
        } else {
            self.selected_control = 0;
        }
    }

    fn toggle_selected_control_focus(&mut self) {
        self.controls[self.selected_control] = self.get_selected_control().toggle_focus();
    }

    fn adjust_focused_percentage_control_by_delta(&mut self, delta: i8) {
        if let Some(AdjustableControl::Percentage(focused, value)) =
            self.get_selected_and_focused_control()
        {
            let new_value = *value as i8 + delta;

            if (0..=100).contains(&new_value) {
                self.controls[self.selected_control] =
                    AdjustableControl::Percentage(*focused, new_value as u8);
            }
        }
    }

    fn set_percentage_control_by_index(&mut self, index: usize, control: AdjustableControl) {
        self.controls[index] = control;
    }

    fn get_selected_and_focused_control(&self) -> Option<&AdjustableControl> {
        let selected = self.get_selected_control();

        if selected.is_focused() {
            Some(selected)
        } else {
            None
        }
    }

    fn get_selected_control(&self) -> &AdjustableControl {
        &self.controls[self.selected_control]
    }

    fn is_panel_selected_and_control_focused_by_index(&self, index: usize) -> bool {
        self.selected && self.selected_control == index && self.get_selected_control().is_focused()
    }

    fn adjustable_control_style(&self, selected: Style, default: Style, index: usize) -> Style {
        if self.selected && self.selected_control == index {
            selected
        } else {
            default
        }
    }

    fn borders_style(&self, theme: &Theme) -> Style {
        if self.selected {
            Style::default().fg(theme.border_active).bold()
        } else {
            Style::default().fg(theme.border)
        }
    }
}
