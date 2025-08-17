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

pub struct BrightnessPanelComponent {
    selected: bool,
}

impl BrightnessPanelComponent {
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

    fn render_fp_brightness(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let gauge = match controls.fp_brightness_percentage() {
            Some(fp_brightness_percentage) => Gauge::default()
                .percent(fp_brightness_percentage as u16)
                .gauge_style(Style::new().yellow()),
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(Paragraph::new("Fingerprint LED brigtness"), key_area);

        match controls.fp_brightness_level() {
            Some(fp_brightness_level) => {
                let [gauge_area, level_area] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                        .areas(value_area);

                let fp_brightness_level_text = format!("{:?}", fp_brightness_level);

                frame.render_widget(gauge, gauge_area);
                frame.render_widget(Paragraph::new(fp_brightness_level_text), level_area);
            }
            None => {
                frame.render_widget(gauge, value_area);
            }
        }
    }

    fn render_kb_brightness(
        &self,
        frame: &mut Frame,
        key_area: Rect,
        value_area: Rect,
        controls: &FrameworkControls,
    ) {
        let gauge = match controls.kb_brightness_percentage() {
            Some(kb_brightness_percentage) => Gauge::default()
                .percent(kb_brightness_percentage as u16)
                .gauge_style(Style::new().yellow()),
            None => Gauge::default().percent(0).label("N/A"),
        };

        frame.render_widget(Paragraph::new("Keyboard brightness"), key_area);
        frame.render_widget(gauge, value_area);
    }
}

impl SelectableComponent for BrightnessPanelComponent {
    fn toggle(&mut self) {
        self.selected = !self.selected;
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Component for BrightnessPanelComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect, controls: &FrameworkControls) {
        let block = Block::default()
            .title(" Brightness ")
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

        let [fp_brightness_key_area, kb_brightness_key_area] =
            Layout::vertical([Constraint::Max(1), Constraint::Max(1)])
                .spacing(1)
                .areas(keys_block.inner(keys_area));
        let [fp_brightness_value_area, kb_brightness_value_area] =
            Layout::vertical([Constraint::Max(1), Constraint::Max(1)])
                .spacing(1)
                .areas(values_block.inner(values_area));

        // Fingerprint brightness
        self.render_fp_brightness(
            frame,
            fp_brightness_key_area,
            fp_brightness_value_area,
            controls,
        );

        // Keyboard brightness
        self.render_kb_brightness(
            frame,
            kb_brightness_key_area,
            kb_brightness_value_area,
            controls,
        );

        // Render blocks
        frame.render_widget(keys_block, keys_area);
        frame.render_widget(values_block, values_area);

        frame.render_widget(block, area);
    }
}
