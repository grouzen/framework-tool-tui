use bgraph::{ColorGradient, GradientMode, Graph, TimeSeriesState};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    framework::info::FrameworkInfo,
    tui::{component::Component, theme::Theme},
};

const HISTORY_SIZE: usize = 200;

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
            voltage_series: TimeSeriesState::with_range(HISTORY_SIZE, 0.0, 20.0),
            current_series: TimeSeriesState::with_range(HISTORY_SIZE, 0.0, 5.0),
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

        // Create block with border
        let block = Block::default()
            .borders(Borders::NONE)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border))
            .style(Style::default().bg(theme.background));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Split area for voltage (top) and current (bottom) graphs
        let [voltage_area, current_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(inner_area);

        // Create gradient (blue → orange → red)
        let gradient = ColorGradient::three_point(
            ratatui::style::Color::Blue,
            ratatui::style::Color::Rgb(255, 165, 0), // Orange
            ratatui::style::Color::Red,
        );

        // Render voltage graph (top)
        let voltage_graph = Graph::new(&self.voltage_series)
            .x_range(0.0, 1.0)
            .y_range(0.0, 20.0)
            .gradient(gradient.clone())
            .gradient_mode(GradientMode::Position);
        frame.render_widget(voltage_graph, voltage_area);

        // Render current graph (bottom, mirrored using invert_y)
        let current_graph = Graph::new(&self.current_series)
            .x_range(0.0, 1.0)
            .y_range(0.0, 5.0)
            .gradient(gradient)
            .gradient_mode(GradientMode::Position)
            .invert_y(true);
        frame.render_widget(current_graph, current_area);
    }
}
