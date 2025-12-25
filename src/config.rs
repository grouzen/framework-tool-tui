use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::tui::theme::ThemeVariant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: ThemeVariant,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeVariant::Framework,
        }
    }
}

impl Config {
    /// Get the config file path
    fn config_path() -> color_eyre::Result<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not determine local data directory"))?;

        let config_dir = data_dir.join("framework-tool-tui");
        fs::create_dir_all(&config_dir)?;

        Ok(config_dir.join("config.toml"))
    }

    /// Load configuration from file, or create default if it doesn't exist
    pub fn load() -> color_eyre::Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // First startup - create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> color_eyre::Result<()> {
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// Update the theme and save
    pub fn set_theme(&mut self, theme: ThemeVariant) -> color_eyre::Result<()> {
        self.theme = theme;
        self.save()
    }
}
