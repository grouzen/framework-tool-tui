use ratatui::{
    crossterm::event::Event,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    app::AppEvent,
    framework::info::FrameworkInfo,
    tui::{
        component::{
            charge_graph_panel::ChargeGraphPanelComponent, charge_panel::ChargePanelComponent,
            AdjustableComponent, AdjustablePanel, Component,
        },
        theme::Theme,
    },
};

pub struct ChargePanelsComponent {
    graph_panel: ChargeGraphPanelComponent,
    charge_panel: ChargePanelComponent,
}

impl Default for ChargePanelsComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargePanelsComponent {
    pub fn new() -> Self {
        Self {
            graph_panel: ChargeGraphPanelComponent::new(),
            charge_panel: ChargePanelComponent::new(),
        }
    }
}

impl Component for ChargePanelsComponent {
    fn handle_input(&mut self, event: Event) -> Option<AppEvent> {
        // Forward input to the charge panel
        self.charge_panel.handle_input(event)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme, info: &FrameworkInfo) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border))
            .style(Style::default().bg(theme.background));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Split horizontally: graph on left (fill), charge panel on right (fixed width)
        let [graph_area, charge_panel_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Max(55)]).areas(inner_area);

        // Render graph panel on the left
        self.graph_panel.render(frame, graph_area, theme, info);

        // Render charge panel on the right
        self.charge_panel
            .render(frame, charge_panel_area, theme, info);
    }
}

impl AdjustableComponent for ChargePanelsComponent {
    fn panel(&mut self) -> &mut AdjustablePanel {
        // Forward to the charge panel's adjustable panel
        self.charge_panel.panel()
    }
}
