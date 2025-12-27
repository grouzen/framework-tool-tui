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
    MonochromeDark,
    MonochromeLight,
    MonokaiPro,
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
            "monochrome_dark" => Ok(ThemeVariant::MonochromeDark),
            "monochrome_light" => Ok(ThemeVariant::MonochromeLight),
            "monokai_pro" => Ok(ThemeVariant::MonokaiPro),
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
            ThemeVariant::MonochromeDark => "Monochrome Dark",
            ThemeVariant::MonochromeLight => "Monochrome Light",
            ThemeVariant::MonokaiPro => "Monokai Pro",
        }
    }

    pub const ALL: [ThemeVariant; 12] = [
        ThemeVariant::Framework,
        ThemeVariant::Alucard,
        ThemeVariant::Dracula,
        ThemeVariant::CatppuccinFrappe,
        ThemeVariant::CatppuccinLatte,
        ThemeVariant::CatppuccinMacchiato,
        ThemeVariant::CatppuccinMocha,
        ThemeVariant::GithubDark,
        ThemeVariant::GithubLight,
        ThemeVariant::MonochromeDark,
        ThemeVariant::MonochromeLight,
        ThemeVariant::MonokaiPro,
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
    pub charge_bar_background: Color,
    pub charge_level_background: Color,
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
            ThemeVariant::MonochromeDark => Self::monochrome_dark(),
            ThemeVariant::MonochromeLight => Self::monochrome_light(),
            ThemeVariant::MonokaiPro => Self::monokai_pro(),
        }
    }

    pub fn framework() -> Self {
        Self {
            variant: ThemeVariant::Framework,
            text: Color::from_str("#F5F5F5").unwrap(),
            background: Color::from_str("#1F1F1F").unwrap(),
            border: Color::from_str("#F45A27").unwrap(),
            border_active: Color::from_str("#FFD600").unwrap(),
            indication_ok: Color::from_str("#00B16A").unwrap(),
            indication_warning: Color::from_str("#E53935").unwrap(),
            brightness_bar: Color::from_str("#fdbe54").unwrap(),
            charge_bar: Color::from_str("#9481D8").unwrap(),
            charge_bar_background: Color::from_str("#363636").unwrap(),
            charge_level_background: Color::from_str("#363636").unwrap(),
            highlighted_text: Color::from_str("#FF7447").unwrap(),
            informative_text: Color::from_str("#9481D8").unwrap(),
        }
    }

    pub fn alucard() -> Self {
        Self {
            variant: ThemeVariant::Alucard,
            text: Color::from_str("#1F1F1F").unwrap(),
            background: Color::from_str("#FFFBEB").unwrap(),
            border: Color::from_str("#A34D14").unwrap(),
            border_active: Color::from_str("#846E15").unwrap(),
            indication_ok: Color::from_str("#14710A").unwrap(),
            indication_warning: Color::from_str("#CB3A2A").unwrap(),
            brightness_bar: Color::from_str("#846E15").unwrap(),
            charge_bar: Color::from_str("#644AC9").unwrap(),
            charge_bar_background: Color::from_str("#CFCFDE").unwrap(),
            charge_level_background: Color::from_str("#CFCFDE").unwrap(),
            highlighted_text: Color::from_str("#A34D14").unwrap(),
            informative_text: Color::from_str("#644AC9").unwrap(),
        }
    }

    pub fn dracula() -> Self {
        Self {
            variant: ThemeVariant::Dracula,
            text: Color::from_str("#F8F8F2").unwrap(),
            background: Color::from_str("#282A36").unwrap(),
            border: Color::from_str("#BD93F9").unwrap(),
            border_active: Color::from_str("#FFB86C").unwrap(),
            indication_ok: Color::from_str("#50FA7B").unwrap(),
            indication_warning: Color::from_str("#FF5555").unwrap(),
            brightness_bar: Color::from_str("#F1FA8C").unwrap(),
            charge_bar: Color::from_str("#BD93F9").unwrap(),
            charge_bar_background: Color::from_str("#44475A").unwrap(),
            charge_level_background: Color::from_str("#44475A").unwrap(),
            highlighted_text: Color::from_str("#FF79C6").unwrap(),
            informative_text: Color::from_str("#8BE9FD").unwrap(),
        }
    }

    pub fn catppuccin_frappe() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinFrappe,
            text: Color::from_str("#C6D0F5").unwrap(),
            background: Color::from_str("#232634").unwrap(),
            border: Color::from_str("#EF9F76").unwrap(),
            border_active: Color::from_str("#E5C890").unwrap(),
            indication_ok: Color::from_str("#A6D189").unwrap(),
            indication_warning: Color::from_str("#E78284").unwrap(),
            brightness_bar: Color::from_str("#E5C890").unwrap(),
            charge_bar: Color::from_str("#8CAAEE").unwrap(),
            charge_bar_background: Color::from_str("#303446").unwrap(),
            charge_level_background: Color::from_str("#303446").unwrap(),
            highlighted_text: Color::from_str("#EF9F76").unwrap(),
            informative_text: Color::from_str("#CA9EE6").unwrap(),
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinLatte,
            text: Color::from_str("#4C4F69").unwrap(),
            background: Color::from_str("#DCE0E8").unwrap(),
            border: Color::from_str("#D20F39").unwrap(),
            border_active: Color::from_str("#DF8E1D").unwrap(),
            indication_ok: Color::from_str("#40A02B").unwrap(),
            indication_warning: Color::from_str("#D20F39").unwrap(),
            brightness_bar: Color::from_str("#DF8E1D").unwrap(),
            charge_bar: Color::from_str("#1E66F5").unwrap(),
            charge_bar_background: Color::from_str("#EFF1F5").unwrap(),
            charge_level_background: Color::from_str("#EFF1F5").unwrap(),
            highlighted_text: Color::from_str("#FE640B").unwrap(),
            informative_text: Color::from_str("#8839EF").unwrap(),
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinMacchiato,
            text: Color::from_str("#CAD3F5").unwrap(),
            background: Color::from_str("#181926").unwrap(),
            border: Color::from_str("#F5A97F").unwrap(),
            border_active: Color::from_str("#EED49F").unwrap(),
            indication_ok: Color::from_str("#A6DA95").unwrap(),
            indication_warning: Color::from_str("#ED8796").unwrap(),
            brightness_bar: Color::from_str("#EED49F").unwrap(),
            charge_bar: Color::from_str("#8AADF4").unwrap(),
            charge_bar_background: Color::from_str("#24273A").unwrap(),
            charge_level_background: Color::from_str("#24273A").unwrap(),
            highlighted_text: Color::from_str("#F5A97F").unwrap(),
            informative_text: Color::from_str("#C6A0F6").unwrap(),
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinMocha,
            text: Color::from_str("#CDD6F4").unwrap(),
            background: Color::from_str("#11111B").unwrap(),
            border: Color::from_str("#FAB387").unwrap(),
            border_active: Color::from_str("#F9E2AF").unwrap(),
            indication_ok: Color::from_str("#A6E3A1").unwrap(),
            indication_warning: Color::from_str("#F38BA8").unwrap(),
            brightness_bar: Color::from_str("#F9E2AF").unwrap(),
            charge_bar: Color::from_str("#89B4FA").unwrap(),
            charge_bar_background: Color::from_str("#1E1E2E").unwrap(),
            charge_level_background: Color::from_str("#1E1E2E").unwrap(),
            highlighted_text: Color::from_str("#FAB387").unwrap(),
            informative_text: Color::from_str("#CBA6F7").unwrap(),
        }
    }

    pub fn github_dark() -> Self {
        Self {
            variant: ThemeVariant::GithubDark,
            text: Color::from_str("#D1D7E0").unwrap(),
            background: Color::from_str("#212830").unwrap(),
            border: Color::from_str("#FF8E40").unwrap(),
            border_active: Color::from_str("#D3FA37").unwrap(),
            indication_ok: Color::from_str("#5fED83").unwrap(),
            indication_warning: Color::from_str("#FF8E40").unwrap(),
            brightness_bar: Color::from_str("#D3FA37").unwrap(),
            charge_bar: Color::from_str("#1F6FEB").unwrap(),
            charge_bar_background: Color::from_str("#262C36").unwrap(),
            charge_level_background: Color::from_str("#262C36").unwrap(),
            highlighted_text: Color::from_str("#9EECFF").unwrap(),
            informative_text: Color::from_str("#FF80D2").unwrap(),
        }
    }

    pub fn github_light() -> Self {
        Self {
            variant: ThemeVariant::GithubLight,
            text: Color::from_str("#000000").unwrap(),
            background: Color::from_str("#FFFFFF").unwrap(),
            border: Color::from_str("#703100").unwrap(),
            border_active: Color::from_str("#DB9D00").unwrap(),
            indication_ok: Color::from_str("#1f883D").unwrap(),
            indication_warning: Color::from_str("#703100").unwrap(),
            brightness_bar: Color::from_str("#DB9D00").unwrap(),
            charge_bar: Color::from_str("#0969DA").unwrap(),
            charge_bar_background: Color::from_str("#F6F8FA").unwrap(),
            charge_level_background: Color::from_str("#F6F8FA").unwrap(),
            highlighted_text: Color::from_str("#212183").unwrap(),
            informative_text: Color::from_str("#8342FA").unwrap(),
        }
    }

    pub fn monochrome_dark() -> Self {
        Self {
            variant: ThemeVariant::MonochromeDark,
            text: Color::from_str("#FFFFFF").unwrap(),
            background: Color::from_str("#000000").unwrap(),
            border: Color::from_str("#FFFFFF").unwrap(),
            border_active: Color::from_str("#FFFFFF").unwrap(),
            indication_ok: Color::from_str("#FFFFFF").unwrap(),
            indication_warning: Color::from_str("#FFFFFF").unwrap(),
            brightness_bar: Color::from_str("#FFFFFF").unwrap(),
            charge_bar: Color::from_str("#FFFFFF").unwrap(),
            charge_bar_background: Color::from_str("#000000").unwrap(),
            charge_level_background: Color::from_str("#000000").unwrap(),
            highlighted_text: Color::from_str("#FFFFFF").unwrap(),
            informative_text: Color::from_str("#FFFFFF").unwrap(),
        }
    }

    pub fn monochrome_light() -> Self {
        Self {
            variant: ThemeVariant::MonochromeLight,
            text: Color::from_str("#000000").unwrap(),
            background: Color::from_str("#FFFFFF").unwrap(),
            border: Color::from_str("#000000").unwrap(),
            border_active: Color::from_str("#000000").unwrap(),
            indication_ok: Color::from_str("#000000").unwrap(),
            indication_warning: Color::from_str("#000000").unwrap(),
            brightness_bar: Color::from_str("#000000").unwrap(),
            charge_bar: Color::from_str("#000000").unwrap(),
            charge_bar_background: Color::from_str("#FFFFFF").unwrap(),
            charge_level_background: Color::from_str("#FFFFFF").unwrap(),
            highlighted_text: Color::from_str("#000000").unwrap(),
            informative_text: Color::from_str("#000000").unwrap(),
        }
    }

    pub fn monokai_pro() -> Self {
        Self {
            variant: ThemeVariant::MonokaiPro,
            text: Color::from_str("#FFFFFF").unwrap(),
            background: Color::from_str("#161517").unwrap(),
            border: Color::from_str("#FC9867").unwrap(),
            border_active: Color::from_str("#FFD866").unwrap(),
            indication_ok: Color::from_str("#A9DC76").unwrap(),
            indication_warning: Color::from_str("#FF6188").unwrap(),
            brightness_bar: Color::from_str("#FFD866").unwrap(),
            charge_bar: Color::from_str("#AB9DF2").unwrap(),
            charge_bar_background: Color::from_str("#373138").unwrap(),
            charge_level_background: Color::from_str("#373138").unwrap(),
            highlighted_text: Color::from_str("#FC9867").unwrap(),
            informative_text: Color::from_str("#AB9DF2").unwrap(),
        }
    }
}
