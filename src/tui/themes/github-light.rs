// Github Light
// https://github.com/primer/github-vscode-theme/blob/main/src/classic/colors.json
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
            background: Color::from_str("#FFFFFF").unwrap(), // Black background
            border: Color::from_str("#A04100").unwrap(), // Orange non active
            border_active: Color::from_str("#735C0F").unwrap(), // Yellow Active border
            indication_ok: Color::from_str("#144620").unwrap(), // Green charged
            indication_warning: Color::from_str("#86181D").unwrap(), // Red off privacy
            brightness_bar: Color::from_str("#735C0F").unwrap(),  // Yellow brightness
            charge_bar: Color::from_str("#29134E").unwrap(), // Purple max charge
            highlighted_text: Color::from_str("#A04100").unwrap(), // Orange version tag
            informative_text: Color::from_str("#29134E").unwrap(), // Purple info text
        }
    }
}
