use bgraph::{ColorGradient, GradientMode, Graph, TimeSeriesState};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    framework::info::FrameworkInfo,
    tui::{component::Component, theme::Theme},
};

const HISTORY_SIZE: usize = 200;

const VOLTAGE_MAX: f32 = 20.0; // Max voltage in volts

const CURRENT_MAX: f32 = 5.0; // Max current in amps

pub struct ChargeGraphPanelComponent {
    voltage_series: TimeSeriesState,
    current_series: TimeSeriesState,
}

impl Default for ChargeGraphPanelComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargeGraphPanelComponent {
    pub fn new() -> Self {
        Self {
            voltage_series: TimeSeriesState::with_range(HISTORY_SIZE, 0.0, VOLTAGE_MAX),
            current_series: TimeSeriesState::with_range(HISTORY_SIZE, 0.0, CURRENT_MAX),
        }
    }

    fn update_history(&mut self, info: &FrameworkInfo) {
        // Convert mV to V, use 0.0 as default
        let voltage = info
            .charger_voltage
            .map(|v| v as f32 / 1000.0)
            .unwrap_or(0.0);

        // Convert mA to A, use 0.0 as default
        let current = info
            .charger_current
            .map(|c| c as f32 / 1000.0)
            .unwrap_or(0.0);

        // Add to series
        self.voltage_series.push(voltage);
        self.current_series.push(current);
    }
}

impl Component for ChargeGraphPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        // Update series with current data
        self.update_history(info);

        let [area] = Layout::vertical([Constraint::Min(0)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::default().borders(Borders::NONE);
        let inner_area = block.inner(area);

        frame.render_widget(block, area);

        // Split area for voltage (top) and current (bottom) graphs
        let [voltage_area, current_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(inner_area);

        let voltage_gradient = ColorGradient::two_point(
            theme.charge_voltage_graph_light,
            theme.charge_voltage_graph_dark,
        );
        let current_gradient = ColorGradient::two_point(
            theme.charge_current_graph_dark,
            theme.charge_current_graph_light,
        );

        // Render voltage graph (top)
        let voltage_graph = Graph::new(&self.voltage_series)
            .x_range(0.0, 1.0)
            .y_range(0.0, VOLTAGE_MAX)
            .gradient(voltage_gradient)
            .gradient_mode(GradientMode::Position);

        // Render current graph (bottom, mirrored using invert_y)
        let current_graph = Graph::new(&self.current_series)
            .x_range(0.0, 1.0)
            .y_range(0.0, CURRENT_MAX)
            .gradient(current_gradient)
            .gradient_mode(GradientMode::Position)
            .invert_y(true);

        frame.render_widget(voltage_graph, voltage_area);
        frame.render_widget(current_graph, current_area);
    }
}
