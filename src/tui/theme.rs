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
    pub text: Color,
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
            text: Color::White,
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
            text: Color::Black,
            background: Color::from_str("#FFFBEB").unwrap(),
            border: Color::from_str("#A34D14").unwrap(),
            border_active: Color::from_str("#846E15").unwrap(),
            indication_ok: Color::from_str("#14710A").unwrap(),
            indication_warning: Color::from_str("#CB3A2A").unwrap(),
            brightness_bar: Color::from_str("#846E15").unwrap(),
            charge_bar: Color::from_str("#644AC9").unwrap(),
            highlighted_text: Color::from_str("#A34D14").unwrap(),
            informative_text: Color::from_str("#644AC9").unwrap(),
        }
    }

    pub fn dracula() -> Self {
        Self {
            variant: ThemeVariant::Dracula,
            text: Color::White,
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
            text: Color::White,
            background: Color::from_str("#232634").unwrap(),
            border: Color::from_str("#ef9f76").unwrap(),
            border_active: Color::from_str("#e5c890").unwrap(),
            indication_ok: Color::from_str("#a6d189").unwrap(),
            indication_warning: Color::from_str("#e78284").unwrap(),
            brightness_bar: Color::from_str("#e5c890").unwrap(),
            charge_bar: Color::from_str("#ca9ee6").unwrap(),
            highlighted_text: Color::from_str("#ef9f76").unwrap(),
            informative_text: Color::from_str("#ca9ee6").unwrap(),
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinLatte,
            text: Color::Black,
            background: Color::from_str("#dce0e8").unwrap(),
            border: Color::from_str("#fe640b").unwrap(),
            border_active: Color::from_str("#df8e1d").unwrap(),
            indication_ok: Color::from_str("#40a02b").unwrap(),
            indication_warning: Color::from_str("#d20f39").unwrap(),
            brightness_bar: Color::from_str("#df8e1d").unwrap(),
            charge_bar: Color::from_str("#8839ef").unwrap(),
            highlighted_text: Color::from_str("#fe640b").unwrap(),
            informative_text: Color::from_str("#8839ef").unwrap(),
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinMacchiato,
            text: Color::White,
            background: Color::from_str("#181926").unwrap(),
            border: Color::from_str("#f5a97f").unwrap(),
            border_active: Color::from_str("#eed49f").unwrap(),
            indication_ok: Color::from_str("#a6da95").unwrap(),
            indication_warning: Color::from_str("#ed8796").unwrap(),
            brightness_bar: Color::from_str("#eed49f").unwrap(),
            charge_bar: Color::from_str("#c6a0f6").unwrap(),
            highlighted_text: Color::from_str("#f5a97f").unwrap(),
            informative_text: Color::from_str("#c6a0f6").unwrap(),
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinMocha,
            text: Color::White,
            background: Color::from_str("#11111b").unwrap(),
            border: Color::from_str("#fab387").unwrap(),
            border_active: Color::from_str("#f9e2af").unwrap(),
            indication_ok: Color::from_str("#a6e3a1").unwrap(),
            indication_warning: Color::from_str("#f38ba8").unwrap(),
            brightness_bar: Color::from_str("#f9e2af").unwrap(),
            charge_bar: Color::from_str("#cba6f7").unwrap(),
            highlighted_text: Color::from_str("#fab387").unwrap(),
            informative_text: Color::from_str("#cba6f7").unwrap(),
        }
    }

    pub fn github_dark() -> Self {
        Self {
            variant: ThemeVariant::GithubDark,
            text: Color::White,
            background: Color::from_str("#1B1F23").unwrap(),
            border: Color::from_str("#FFF8F2").unwrap(),
            border_active: Color::from_str("#FFFDEF").unwrap(),
            indication_ok: Color::from_str("#F0FFF4").unwrap(),
            indication_warning: Color::from_str("#FFEEF0").unwrap(),
            brightness_bar: Color::from_str("#FFFDEF").unwrap(),
            charge_bar: Color::from_str("#F5F0FF").unwrap(),
            highlighted_text: Color::from_str("#FFF8F2").unwrap(),
            informative_text: Color::from_str("#F5F0FF").unwrap(),
        }
    }

    pub fn github_light() -> Self {
        Self {
            variant: ThemeVariant::GithubLight,
            text: Color::Black,
            background: Color::from_str("#FFFFFF").unwrap(),
            border: Color::from_str("#A04100").unwrap(),
            border_active: Color::from_str("#735C0F").unwrap(),
            indication_ok: Color::from_str("#144620").unwrap(),
            indication_warning: Color::from_str("#86181D").unwrap(),
            brightness_bar: Color::from_str("#735C0F").unwrap(),
            charge_bar: Color::from_str("#29134E").unwrap(),
            highlighted_text: Color::from_str("#A04100").unwrap(),
            informative_text: Color::from_str("#29134E").unwrap(),
        }
    }

    pub fn monokai_pro_dark() -> Self {
        Self {
            variant: ThemeVariant::MonokaiProDark,
            text: Color::White,
            background: Color::from_str("#221F22").unwrap(),
            border: Color::from_str("#FC9867").unwrap(),
            border_active: Color::from_str("#FFD866").unwrap(),
            indication_ok: Color::from_str("#A9DC76").unwrap(),
            indication_warning: Color::from_str("#FF6188").unwrap(),
            brightness_bar: Color::from_str("#FFD866").unwrap(),
            charge_bar: Color::from_str("#ab9df2").unwrap(),
            highlighted_text: Color::from_str("#FC9867").unwrap(),
            informative_text: Color::from_str("#ab9df2").unwrap(),
        }
    }

    pub fn monokai_pro_light() -> Self {
        Self {
            variant: ThemeVariant::MonokaiProLight,
            text: Color::Black,
            background: Color::from_str("#FFFFFF").unwrap(),
            border: Color::from_str("#FC9768").unwrap(),
            border_active: Color::from_str("#FFD866").unwrap(),
            indication_ok: Color::from_str("#a9dc77").unwrap(),
            indication_warning: Color::from_str("#ff6189").unwrap(),
            brightness_bar: Color::from_str("#FFD866").unwrap(),
            charge_bar: Color::from_str("#AB9DF2").unwrap(),
            highlighted_text: Color::from_str("#FC9768").unwrap(),
            informative_text: Color::from_str("#AB9DF2").unwrap(),
        }
    }
}
