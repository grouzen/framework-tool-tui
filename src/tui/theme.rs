use std::str::FromStr;

use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    Framework,
    Dracula,
    Nord,
    Gruvbox,
}

impl ThemeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::Framework => "Framework",
            ThemeVariant::Dracula => "Dracula",
            ThemeVariant::Nord => "Nord",
            ThemeVariant::Gruvbox => "Gruvbox",
        }
    }

    pub const ALL: [ThemeVariant; 4] = [
        ThemeVariant::Framework,
        ThemeVariant::Dracula,
        ThemeVariant::Nord,
        ThemeVariant::Gruvbox,
    ];

    pub fn next(&self) -> Self {
        let current_idx = Self::ALL.iter().position(|t| t == self).unwrap();
        Self::ALL[(current_idx + 1) % Self::ALL.len()]
    }

    pub fn previous(&self) -> Self {
        let current_idx = Self::ALL.iter().position(|t| t == self).unwrap();
        let prev_idx = if current_idx == 0 {
            Self::ALL.len() - 1
        } else {
            current_idx - 1
        };
        Self::ALL[prev_idx]
    }
}

pub struct Theme {
    pub variant: ThemeVariant,
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
        Theme::from_variant(ThemeVariant::Framework)
    }
}

impl Theme {
    pub fn from_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::Framework => Self::framework(),
            ThemeVariant::Dracula => Self::dracula(),
            ThemeVariant::Nord => Self::nord(),
            ThemeVariant::Gruvbox => Self::gruvbox(),
        }
    }

    pub fn framework() -> Self {
        Self {
            variant: ThemeVariant::Framework,
            background: Color::Black,
            border: Color::from_str("#FF7447").unwrap(),
            border_active: Color::from_str("#FFD600").unwrap(),
            indication_ok: Color::from_str("#00B16A").unwrap(),
            indication_warning: Color::from_str("#E53935").unwrap(),
            brightness_bar: Color::from_str("#FFD600").unwrap(),
            charge_bar: Color::from_str("#9481D8").unwrap(),
            highlighted_text: Color::from_str("#FF7447").unwrap(),
            informative_text: Color::from_str("#9481D8").unwrap(),
        }
    }

    pub fn dracula() -> Self {
        Self {
            variant: ThemeVariant::Dracula,
            background: Color::from_str("#282a36").unwrap(),
            border: Color::from_str("#bd93f9").unwrap(),
            border_active: Color::from_str("#ffb86c").unwrap(),
            indication_ok: Color::from_str("#50fa7b").unwrap(),
            indication_warning: Color::from_str("#ff5555").unwrap(),
            brightness_bar: Color::from_str("#f1fa8c").unwrap(),
            charge_bar: Color::from_str("#bd93f9").unwrap(),
            highlighted_text: Color::from_str("#ff79c6").unwrap(),
            informative_text: Color::from_str("#8be9fd").unwrap(),
        }
    }

    pub fn nord() -> Self {
        Self {
            variant: ThemeVariant::Nord,
            background: Color::from_str("#2e3440").unwrap(),
            border: Color::from_str("#88c0d0").unwrap(),
            border_active: Color::from_str("#ebcb8b").unwrap(),
            indication_ok: Color::from_str("#a3be8c").unwrap(),
            indication_warning: Color::from_str("#bf616a").unwrap(),
            brightness_bar: Color::from_str("#ebcb8b").unwrap(),
            charge_bar: Color::from_str("#b48ead").unwrap(),
            highlighted_text: Color::from_str("#81a1c1").unwrap(),
            informative_text: Color::from_str("#8fbcbb").unwrap(),
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            variant: ThemeVariant::Gruvbox,
            background: Color::from_str("#282828").unwrap(),
            border: Color::from_str("#fe8019").unwrap(),
            border_active: Color::from_str("#fabd2f").unwrap(),
            indication_ok: Color::from_str("#b8bb26").unwrap(),
            indication_warning: Color::from_str("#fb4934").unwrap(),
            brightness_bar: Color::from_str("#fabd2f").unwrap(),
            charge_bar: Color::from_str("#d3869b").unwrap(),
            highlighted_text: Color::from_str("#fe8019").unwrap(),
            informative_text: Color::from_str("#83a598").unwrap(),
        }
    }
}
