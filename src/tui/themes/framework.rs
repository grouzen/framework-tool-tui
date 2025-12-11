// Framework
// https://frame.work
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
            background: Color::Black,
            border: Color::from_str("#FF7447").unwrap(), // Orange non active
            border_active: Color::from_str("#FFD600").unwrap(), // Yellow Active border
            indication_ok: Color::from_str("#00B16A").unwrap(), // Green charged
            indication_warning: Color::from_str("#E53935").unwrap(), // Red off privacy
            brightness_bar: Color::from_str("#FFD600").unwrap(),  // Yellow brightness
            charge_bar: Color::from_str("#9481D8").unwrap(), // Purple max charge
            highlighted_text: Color::from_str("#FF7447").unwrap(), // Orange version tag
            informative_text: Color::from_str("#9481D8").unwrap(), // Purple info text
        }
    }
}
