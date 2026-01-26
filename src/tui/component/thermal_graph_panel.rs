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

const FAN_RPM_MAX: f32 = 6000.0;

pub struct ThermalGraphPanelComponent {
    fan_rpm_series: TimeSeriesState,
}

impl Default for ThermalGraphPanelComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalGraphPanelComponent {
    pub fn new() -> Self {
        Self {
            fan_rpm_series: TimeSeriesState::with_range(HISTORY_SIZE, 0.0, FAN_RPM_MAX),
        }
    }

    fn update_history(&mut self, info: &FrameworkInfo) {
        let rpm = info
            .fan_rpm
            .as_ref()
            .and_then(|rpms| rpms.first())
            .map(|&r| r as f32)
            .unwrap_or(0.0);
        
        self.fan_rpm_series.push(rpm);
    }
}

impl Component for ThermalGraphPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        // Update series with current data
        self.update_history(info);

        let [area] = Layout::vertical([Constraint::Min(0)])
            .flex(Flex::Center)
            .areas(area);

        let block = Block::default().borders(Borders::NONE);
        let inner_area = block.inner(area);

        frame.render_widget(block, area);

        let gradient = ColorGradient::two_point(
            theme.thermal_graph_light,
            theme.thermal_graph_dark,
        );

        // Render FAN RPM graph with fixed range
        let fan_graph = Graph::new(&self.fan_rpm_series)
            .x_range(0.0, 1.0)
            .y_range(0.0, FAN_RPM_MAX)
            .gradient(gradient)
            .gradient_mode(GradientMode::Position);

        frame.render_widget(fan_graph, inner_area);
    }
}
