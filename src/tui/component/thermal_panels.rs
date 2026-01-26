use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    framework::info::FrameworkInfo,
    tui::{
        component::{
            thermal_graph_panel::ThermalGraphPanelComponent,
            thermal_panel::ThermalPanelComponent,
            Component,
        },
        theme::Theme,
    },
};

pub struct ThermalPanelsComponent {
    graph_panel: ThermalGraphPanelComponent,
    thermal_panel: ThermalPanelComponent,
}

impl Default for ThermalPanelsComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalPanelsComponent {
    pub fn new() -> Self {
        Self {
            graph_panel: ThermalGraphPanelComponent::new(),
            thermal_panel: ThermalPanelComponent,
        }
    }
}

impl Component for ThermalPanelsComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border))
            .style(Style::default().bg(theme.background));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Split vertically: graph on top (filling space), info panel on bottom (fixed height)
        let [graph_area, thermal_panel_area] =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Max(30)]).areas(inner_area);

        // Render graph panel on top
        self.graph_panel.render(frame, graph_area, theme, info);

        // Render thermal info panel on bottom
        self.thermal_panel
            .render(frame, thermal_panel_area, theme, info);
    }
}
