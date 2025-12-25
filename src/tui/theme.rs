use std::str::FromStr;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeVariant {
    Framework,
    Alucard,
    Dracula,
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    GithubDark,
    GithubLight,
    MonokaiProDark,
    MonokaiProLight,
}

impl FromStr for ThemeVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "framework" => Ok(ThemeVariant::Framework),
            "alucard" => Ok(ThemeVariant::Alucard),
            "dracula" => Ok(ThemeVariant::Dracula),
            "catppuccin_frappe" => Ok(ThemeVariant::CatppuccinFrappe),
            "catppuccin_latte" => Ok(ThemeVariant::CatppuccinLatte),
            "catppuccin_macchiato" => Ok(ThemeVariant::CatppuccinMacchiato),
            "catppuccin_mocha" => Ok(ThemeVariant::CatppuccinMocha),
            "github_dark" => Ok(ThemeVariant::GithubDark),
            "github_light" => Ok(ThemeVariant::GithubLight),
            "monokai_pro_dark" => Ok(ThemeVariant::MonokaiProDark),
            "monokai_pro_light" => Ok(ThemeVariant::MonokaiProLight),
            _ => Err(format!("Unknown theme: {}", s)),
        }
    }
}

impl ThemeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::Framework => "Framework",
            ThemeVariant::Alucard => "Alucard",
            ThemeVariant::Dracula => "Dracula",
            ThemeVariant::CatppuccinFrappe => "Catppuccin Frappe",
            ThemeVariant::CatppuccinLatte => "Catppuccin Latte",
            ThemeVariant::CatppuccinMacchiato => "Catppuccin Macchiato",
            ThemeVariant::CatppuccinMocha => "Catppuccin Mocha",
            ThemeVariant::GithubDark => "GitHub Dark",
            ThemeVariant::GithubLight => "GitHub Light",
            ThemeVariant::MonokaiProDark => "Monokai Pro Dark",
            ThemeVariant::MonokaiProLight => "Monokai Pro Light",
        }
    }

    pub const ALL: [ThemeVariant; 11] = [
        ThemeVariant::Framework,
        ThemeVariant::Alucard,
        ThemeVariant::Dracula,
        ThemeVariant::CatppuccinFrappe,
        ThemeVariant::CatppuccinLatte,
        ThemeVariant::CatppuccinMacchiato,
        ThemeVariant::CatppuccinMocha,
        ThemeVariant::GithubDark,
        ThemeVariant::GithubLight,
        ThemeVariant::MonokaiProDark,
        ThemeVariant::MonokaiProLight,
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
            ThemeVariant::Alucard => Self::alucard(),
            ThemeVariant::Dracula => Self::dracula(),
            ThemeVariant::CatppuccinFrappe => Self::catppuccin_frappe(),
            ThemeVariant::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeVariant::CatppuccinMacchiato => Self::catppuccin_macchiato(),
            ThemeVariant::CatppuccinMocha => Self::catppuccin_mocha(),
            ThemeVariant::GithubDark => Self::github_dark(),
            ThemeVariant::GithubLight => Self::github_light(),
            ThemeVariant::MonokaiProDark => Self::monokai_pro_dark(),
            ThemeVariant::MonokaiProLight => Self::monokai_pro_light(),
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

    pub fn alucard() -> Self {
        Self {
            variant: ThemeVariant::Alucard,
            background: Color::from_str("#FFFBEB").unwrap(), // Background
            border: Color::from_str("#A34D14").unwrap(),     // Orange
            border_active: Color::from_str("#846E15").unwrap(), // Yellow
            indication_ok: Color::from_str("#14710A").unwrap(), // Green
            indication_warning: Color::from_str("#CB3A2A").unwrap(), // Red
            brightness_bar: Color::from_str("#846E15").unwrap(), // Yellow
            charge_bar: Color::from_str("#644AC9").unwrap(), // Purple
            highlighted_text: Color::from_str("#A34D14").unwrap(), // Orange
            informative_text: Color::from_str("#644AC9").unwrap(), // Purple
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

    pub fn catppuccin_frappe() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinFrappe,
            background: Color::from_str("#232634").unwrap(), // Crust
            border: Color::from_str("#ef9f76").unwrap(),     // Peach
            border_active: Color::from_str("#e5c890").unwrap(), // Yellow
            indication_ok: Color::from_str("#a6d189").unwrap(), // Green
            indication_warning: Color::from_str("#e78284").unwrap(), // Red
            brightness_bar: Color::from_str("#e5c890").unwrap(), // Yellow
            charge_bar: Color::from_str("#ca9ee6").unwrap(), // Mauve
            highlighted_text: Color::from_str("#ef9f76").unwrap(), // Peach
            informative_text: Color::from_str("#ca9ee6").unwrap(), // Mauve
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinLatte,
            background: Color::from_str("#dce0e8").unwrap(), // Crust
            border: Color::from_str("#fe640b").unwrap(),     // Peach
            border_active: Color::from_str("#df8e1d").unwrap(), // Yellow
            indication_ok: Color::from_str("#40a02b").unwrap(), // Green
            indication_warning: Color::from_str("#d20f39").unwrap(), // Red
            brightness_bar: Color::from_str("#df8e1d").unwrap(), // Yellow
            charge_bar: Color::from_str("#8839ef").unwrap(), // Mauve
            highlighted_text: Color::from_str("#fe640b").unwrap(), // Peach
            informative_text: Color::from_str("#8839ef").unwrap(), // Mauve
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinMacchiato,
            background: Color::from_str("#181926").unwrap(), // Crust
            border: Color::from_str("#f5a97f").unwrap(),     // Peach
            border_active: Color::from_str("#eed49f").unwrap(), // Yellow
            indication_ok: Color::from_str("#a6da95").unwrap(), // Green
            indication_warning: Color::from_str("#ed8796").unwrap(), // Red
            brightness_bar: Color::from_str("#eed49f").unwrap(), // Yellow
            charge_bar: Color::from_str("#c6a0f6").unwrap(), // Mauve
            highlighted_text: Color::from_str("#f5a97f").unwrap(), // Peach
            informative_text: Color::from_str("#c6a0f6").unwrap(), // Mauve
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinMocha,
            background: Color::from_str("#11111b").unwrap(), // Crust
            border: Color::from_str("#fab387").unwrap(),     // Peach
            border_active: Color::from_str("#f9e2af").unwrap(), // Yellow
            indication_ok: Color::from_str("#a6e3a1").unwrap(), // Green
            indication_warning: Color::from_str("#f38ba8").unwrap(), // Red
            brightness_bar: Color::from_str("#f9e2af").unwrap(), // Yellow
            charge_bar: Color::from_str("#cba6f7").unwrap(), // Mauve
            highlighted_text: Color::from_str("#fab387").unwrap(), // Peach
            informative_text: Color::from_str("#cba6f7").unwrap(), // Mauve
        }
    }

    pub fn github_dark() -> Self {
        Self {
            variant: ThemeVariant::GithubDark,
            background: Color::from_str("#1B1F23").unwrap(), // Black background
            border: Color::from_str("#FFF8F2").unwrap(),     // Orange non active
            border_active: Color::from_str("#FFFDEF").unwrap(), // Yellow Active border
            indication_ok: Color::from_str("#F0FFF4").unwrap(), // Green charged
            indication_warning: Color::from_str("#FFEEF0").unwrap(), // Red off privacy
            brightness_bar: Color::from_str("#FFFDEF").unwrap(), // Yellow brightness
            charge_bar: Color::from_str("#F5F0FF").unwrap(), // Purple max charge
            highlighted_text: Color::from_str("#FFF8F2").unwrap(), // Orange version tag
            informative_text: Color::from_str("#F5F0FF").unwrap(), // Purple info text
        }
    }

    pub fn github_light() -> Self {
        Self {
            variant: ThemeVariant::GithubLight,
            background: Color::from_str("#FFFFFF").unwrap(), // Black background
            border: Color::from_str("#A04100").unwrap(),     // Orange non active
            border_active: Color::from_str("#735C0F").unwrap(), // Yellow Active border
            indication_ok: Color::from_str("#144620").unwrap(), // Green charged
            indication_warning: Color::from_str("#86181D").unwrap(), // Red off privacy
            brightness_bar: Color::from_str("#735C0F").unwrap(), // Yellow brightness
            charge_bar: Color::from_str("#29134E").unwrap(), // Purple max charge
            highlighted_text: Color::from_str("#A04100").unwrap(), // Orange version tag
            informative_text: Color::from_str("#29134E").unwrap(), // Purple info text
        }
    }

    pub fn monokai_pro_dark() -> Self {
        Self {
            variant: ThemeVariant::MonokaiProDark,
            background: Color::from_str("#221F22").unwrap(), // Background
            border: Color::from_str("#FC9867").unwrap(),     // Orange
            border_active: Color::from_str("#FFD866").unwrap(), // Yellow
            indication_ok: Color::from_str("#A9DC76").unwrap(), // Green
            indication_warning: Color::from_str("#FF6188").unwrap(), // "Red"
            brightness_bar: Color::from_str("#FFD866").unwrap(), // Yellow
            charge_bar: Color::from_str("#ab9df2").unwrap(), // Purple
            highlighted_text: Color::from_str("#FC9867").unwrap(), // Orange
            informative_text: Color::from_str("#ab9df2").unwrap(), // Purple
        }
    }

    pub fn monokai_pro_light() -> Self {
        Self {
            variant: ThemeVariant::MonokaiProLight,
            background: Color::from_str("#FFFFFF").unwrap(), // Background
            border: Color::from_str("#FC9768").unwrap(),     // Orange
            border_active: Color::from_str("#FFD866").unwrap(), // Yellow
            indication_ok: Color::from_str("#a9dc77").unwrap(), // Green
            indication_warning: Color::from_str("#ff6189").unwrap(), // "Red"
            brightness_bar: Color::from_str("#FFD866").unwrap(), // Yellow
            charge_bar: Color::from_str("#AB9DF2").unwrap(), // Purple
            highlighted_text: Color::from_str("#FC9768").unwrap(), // Orange
            informative_text: Color::from_str("#AB9DF2").unwrap(), // Purple
        }
    }

    // pub fn nord() -> Self {
    //     Self {
    //         variant: ThemeVariant::Nord,
    //         background: Color::from_str("#2e3440").unwrap(),
    //         border: Color::from_str("#88c0d0").unwrap(),
    //         border_active: Color::from_str("#ebcb8b").unwrap(),
    //         indication_ok: Color::from_str("#a3be8c").unwrap(),
    //         indication_warning: Color::from_str("#bf616a").unwrap(),
    //         brightness_bar: Color::from_str("#ebcb8b").unwrap(),
    //         charge_bar: Color::from_str("#b48ead").unwrap(),
    //         highlighted_text: Color::from_str("#81a1c1").unwrap(),
    //         informative_text: Color::from_str("#8fbcbb").unwrap(),
    //     }
    // }

    // pub fn gruvbox() -> Self {
    //     Self {
    //         variant: ThemeVariant::Gruvbox,
    //         background: Color::from_str("#282828").unwrap(),
    //         border: Color::from_str("#fe8019").unwrap(),
    //         border_active: Color::from_str("#fabd2f").unwrap(),
    //         indication_ok: Color::from_str("#b8bb26").unwrap(),
    //         indication_warning: Color::from_str("#fb4934").unwrap(),
    //         brightness_bar: Color::from_str("#fabd2f").unwrap(),
    //         charge_bar: Color::from_str("#d3869b").unwrap(),
    //         highlighted_text: Color::from_str("#fe8019").unwrap(),
    //         informative_text: Color::from_str("#83a598").unwrap(),
    //     }
    // }
}
