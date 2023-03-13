use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

lazy_static! {
    static ref MICHEL_CONFIG_PATH: PathBuf =
        compute_michel_config_path().expect("No config path found");
}

pub const MICHEL_CONFIG_FOLDER: &str = "michel";
const MICHEL_CONFIG_FILENAME: &str = "config.toml";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DesktopConfig {
    username: String,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        DesktopConfig {
            username: String::from("michel"),
        }
    }
}

impl DesktopConfig {
    pub fn load() -> Result<DesktopConfig> {
        let config_file_content = fs::read_to_string(MICHEL_CONFIG_PATH.as_path())?;
        Ok(toml::from_str(&config_file_content)?)
    }

    pub fn save(&self) -> Result<()> {
        let config_content = toml::to_string(&self)?;
        let mut f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(MICHEL_CONFIG_PATH.as_path())?;

        f.write_all(config_content.as_bytes())?;
        f.flush()?;
        Ok(())
    }
}

fn compute_michel_config_path() -> Result<PathBuf> {
    if let Ok(config_path) = env::var("XDG_CONFIG_HOME") {
        return Ok(Path::new(&config_path)
            .join(MICHEL_CONFIG_FOLDER)
            .join(MICHEL_CONFIG_FILENAME));
    }

    if let Ok(config_path) = env::var("HOME") {
        return Ok(Path::new(&config_path)
            .join(MICHEL_CONFIG_FOLDER)
            .join(MICHEL_CONFIG_FILENAME));
    }

    Err(anyhow!("No config path found"))
}
