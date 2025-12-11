// Catppuccin Latte
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
            background: Color::from_str("#dce0e8").unwrap(), // Crust
            border: Color::from_str("#fe640b").unwrap(), // Peach
            border_active: Color::from_str("#df8e1d").unwrap(), // Yellow
            indication_ok: Color::from_str("#40a02b").unwrap(), // Green
            indication_warning: Color::from_str("#d20f39").unwrap(), // Red
            brightness_bar: Color::from_str("#df8e1d").unwrap(), // Yellow
            charge_bar: Color::from_str("#8839ef").unwrap(), // Mauve
            highlighted_text: Color::from_str("#fe640b").unwrap(), // Peach
            informative_text: Color::from_str("#8839ef").unwrap(), // Mauve
        }
    }
}
