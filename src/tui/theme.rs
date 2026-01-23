use std::str::FromStr;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeVariant {
    Framework,
    Alucard,
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Dracula,
    GameBoy,
    GithubDark,
    GithubLight,
    GruvboxDark,
    GruvboxLight,
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
            "catppuccin_frappe" => Ok(ThemeVariant::CatppuccinFrappe),
            "catppuccin_latte" => Ok(ThemeVariant::CatppuccinLatte),
            "catppuccin_macchiato" => Ok(ThemeVariant::CatppuccinMacchiato),
            "catppuccin_mocha" => Ok(ThemeVariant::CatppuccinMocha),
            "dracula" => Ok(ThemeVariant::Dracula),
            "gameboy" => Ok(ThemeVariant::GameBoy),
            "github_dark" => Ok(ThemeVariant::GithubDark),
            "github_light" => Ok(ThemeVariant::GithubLight),
            "gruvbox_dark" => Ok(ThemeVariant::GruvboxDark),
            "gruvbox_light" => Ok(ThemeVariant::GruvboxLight),
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
            ThemeVariant::CatppuccinFrappe => "Catppuccin Frappe",
            ThemeVariant::CatppuccinLatte => "Catppuccin Latte",
            ThemeVariant::CatppuccinMacchiato => "Catppuccin Macchiato",
            ThemeVariant::CatppuccinMocha => "Catppuccin Mocha",
            ThemeVariant::Dracula => "Dracula",
            ThemeVariant::GameBoy => "Game Boy",
            ThemeVariant::GithubDark => "GitHub Dark",
            ThemeVariant::GithubLight => "GitHub Light",
            ThemeVariant::GruvboxDark => "Gruvbox Dark",
            ThemeVariant::GruvboxLight => "Gruvbox Light",
            ThemeVariant::MonochromeDark => "Monochrome Dark",
            ThemeVariant::MonochromeLight => "Monochrome Light",
            ThemeVariant::MonokaiPro => "Monokai Pro",
        }
    }

    pub const ALL: [ThemeVariant; 15] = [
        ThemeVariant::Framework,
        ThemeVariant::Alucard,
        ThemeVariant::CatppuccinFrappe,
        ThemeVariant::CatppuccinLatte,
        ThemeVariant::CatppuccinMacchiato,
        ThemeVariant::CatppuccinMocha,
        ThemeVariant::Dracula,
        ThemeVariant::GameBoy,
        ThemeVariant::GithubDark,
        ThemeVariant::GithubLight,
        ThemeVariant::GruvboxDark,
        ThemeVariant::GruvboxLight,
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
    pub bar_background: Color,
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
            ThemeVariant::CatppuccinFrappe => Self::catppuccin_frappe(),
            ThemeVariant::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeVariant::CatppuccinMacchiato => Self::catppuccin_macchiato(),
            ThemeVariant::CatppuccinMocha => Self::catppuccin_mocha(),
            ThemeVariant::Dracula => Self::dracula(),
            ThemeVariant::GameBoy => Self::gameboy(),
            ThemeVariant::GithubDark => Self::github_dark(),
            ThemeVariant::GithubLight => Self::github_light(),
            ThemeVariant::GruvboxDark => Self::gruvbox_dark(),
            ThemeVariant::GruvboxLight => Self::gruvbox_light(),
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
            bar_background: Color::from_str("#363636").unwrap(),
            highlighted_text: Color::from_str("#AEC2C9").unwrap(),
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
            bar_background: Color::from_str("#CFCFDE").unwrap(),
            highlighted_text: Color::from_str("#036A96").unwrap(),
            informative_text: Color::from_str("#644AC9").unwrap(),
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
            bar_background: Color::from_str("#303446").unwrap(),
            highlighted_text: Color::from_str("#8caaee").unwrap(),
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
            bar_background: Color::from_str("#EFF1F5").unwrap(),
            highlighted_text: Color::from_str("#1e66f5").unwrap(),
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
            bar_background: Color::from_str("#24273A").unwrap(),
            highlighted_text: Color::from_str("#8aadf4").unwrap(),
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
            bar_background: Color::from_str("#1E1E2E").unwrap(),
            highlighted_text: Color::from_str("#89b4fa").unwrap(),
            informative_text: Color::from_str("#CBA6f7").unwrap(),
        }
    }

    pub fn dracula() -> Self {
        Self {
            variant: ThemeVariant::Dracula,
            text: Color::from_str("#F8F8F2").unwrap(),
            background: Color::from_str("#282A36").unwrap(),
            border: Color::from_str("#FFB86C").unwrap(),
            border_active: Color::from_str("#FFB86C").unwrap(),
            indication_ok: Color::from_str("#50FA7B").unwrap(),
            indication_warning: Color::from_str("#FF5555").unwrap(),
            brightness_bar: Color::from_str("#F1FA8C").unwrap(),
            charge_bar: Color::from_str("#8BE9FD").unwrap(),
            bar_background: Color::from_str("#44475A").unwrap(),
            highlighted_text: Color::from_str("#8BE9FD").unwrap(),
            informative_text: Color::from_str("#BD93F9").unwrap(),
        }
    }

    pub fn gameboy() -> Self {
        Self {
            variant: ThemeVariant::GameBoy,
            text: Color::from_str("#9A9E3F").unwrap(),
            background: Color::from_str("#1B2A09").unwrap(),
            border: Color::from_str("#496B22").unwrap(),
            border_active: Color::from_str("#9A9E3F").unwrap(),
            indication_ok: Color::from_str("#9A9E3F").unwrap(),
            indication_warning: Color::from_str("#9A9E3F").unwrap(),
            brightness_bar: Color::from_str("#9A9E3F").unwrap(),
            charge_bar: Color::from_str("#9A9E3F").unwrap(),
            bar_background: Color::from_str("#0E450B").unwrap(),
            highlighted_text: Color::from_str("#9A9E3F").unwrap(),
            informative_text: Color::from_str("#9A9E3F").unwrap(),
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
            bar_background: Color::from_str("#262C36").unwrap(),
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
            bar_background: Color::from_str("#F6F8FA").unwrap(),
            highlighted_text: Color::from_str("#212183").unwrap(),
            informative_text: Color::from_str("#8342FA").unwrap(),
        }
    }

    pub fn gruvbox_dark() -> Self {
        Self {
            variant: ThemeVariant::GruvboxDark,
            text: Color::from_str("#EBDBB2").unwrap(),
            background: Color::from_str("#282828").unwrap(),
            border: Color::from_str("#FE8019").unwrap(),
            border_active: Color::from_str("#FABd2F").unwrap(),
            indication_ok: Color::from_str("#B8BB26").unwrap(),
            indication_warning: Color::from_str("#FE8019").unwrap(),
            brightness_bar: Color::from_str("#FABD2F").unwrap(),
            charge_bar: Color::from_str("#458588").unwrap(),
            bar_background: Color::from_str("#504945").unwrap(),
            highlighted_text: Color::from_str("#83A598").unwrap(),
            informative_text: Color::from_str("#D3869B").unwrap(),
        }
    }

    pub fn gruvbox_light() -> Self {
        Self {
            variant: ThemeVariant::GruvboxLight,
            text: Color::from_str("#3C3836").unwrap(),
            background: Color::from_str("#FFFFFF").unwrap(),
            border: Color::from_str("#AF3A03").unwrap(),
            border_active: Color::from_str("#D79921").unwrap(),
            indication_ok: Color::from_str("#79740E").unwrap(),
            indication_warning: Color::from_str("#AF3A03").unwrap(),
            brightness_bar: Color::from_str("#B57614").unwrap(),
            charge_bar: Color::from_str("#458588").unwrap(),
            bar_background: Color::from_str("#D5C4A1").unwrap(),
            highlighted_text: Color::from_str("#076678").unwrap(),
            informative_text: Color::from_str("#8F3F71").unwrap(),
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
            bar_background: Color::from_str("#000000").unwrap(),
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
            bar_background: Color::from_str("#FFFFFF").unwrap(),
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
            bar_background: Color::from_str("#373138").unwrap(),
            highlighted_text: Color::from_str("#77DCE8").unwrap(),
            informative_text: Color::from_str("#AB9DF2").unwrap(),
        }
    }
}
