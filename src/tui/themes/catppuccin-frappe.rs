// Catppuccin Frappe
// https://catppuccin.com
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
            background: Color::from_str("#232634").unwrap(), // Crust
            border: Color::from_str("#ef9f76").unwrap(), // Peach
            border_active: Color::from_str("#e5c890").unwrap(), // Yellow
            indication_ok: Color::from_str("#a6d189").unwrap(), // Green
            indication_warning: Color::from_str("#e78284").unwrap(), // Red
            brightness_bar: Color::from_str("#e5c890").unwrap(), // Yellow
            charge_bar: Color::from_str("#ca9ee6").unwrap(), // Mauve
            highlighted_text: Color::from_str("#ef9f76").unwrap(), // Peach
            informative_text: Color::from_str("#ca9ee6").unwrap(), // Mauve
        }
    }
}
