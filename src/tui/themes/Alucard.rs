// Dracula
// https://draculatheme.com
use std::str::FromStr;

use ratatui::style::Color;

pub struct Theme {
    pub background: Color,
    pub border: Color,
    pub border_active: Color,
    pub indication_ok: Color,
    pub indication_warning: Color,
    pub brightness_bar: Color,
    pub charge_bar: Color,
    pub highlighted_text: Color,
    pub informative_text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::framework()
    }
}

impl Theme {
    pub fn framework() -> Self {
        Self {
            background: Color::from_str("#FFFBEB").unwrap(), // Background
            border: Color::from_str("#A34D14").unwrap(), // Orange
            border_active: Color::from_str("#846E15").unwrap(), // Yellow
            indication_ok: Color::from_str("#14710A").unwrap(), // Green
            indication_warning: Color::from_str("#CB3A2A").unwrap(), // Red
            brightness_bar: Color::from_str("#846E15").unwrap(), // Yellow
            charge_bar: Color::from_str("#644AC9").unwrap(), // Purple
            highlighted_text: Color::from_str("#A34D14").unwrap(), // Orange
            informative_text: Color::from_str("#644AC9").unwrap(), // Purple
        }
    }
}
