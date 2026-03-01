use std::{env, fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{error, template::Template};

fn config_dir(app_name: &str) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            if !appdata.is_empty() {
                return Some(PathBuf::from(appdata).join(app_name));
            }
        }
        return None;
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                return Some(PathBuf::from(xdg).join(app_name));
            }
        }

        if let Ok(home) = env::var("HOME") {
            if !home.is_empty() {
                return Some(PathBuf::from(home).join(".config").join(app_name));
            }
        }
        None
    }
}

fn ensure_config_dir(app_name: &str) -> error::ConfigResult<PathBuf> {
    let dir = config_dir(app_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No config directory available"))?;
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn ensure_templates_file(app_name: &str) -> error::ConfigResult<PathBuf> {
    let dir = ensure_config_dir(app_name)?;
    let path = dir.join("templates.json");

    if !path.exists() {
        let new_templates_config = TemplatesConfig::new();
        let data = serde_json::to_string_pretty(&new_templates_config)?;
        fs::write(&path, data)?;
    }

    Ok(path)
}

pub fn ensure_template_dir(app_name: &str) -> error::ConfigResult<PathBuf> {
    let dir = config_dir(app_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No config directory available"))?;
    fs::create_dir_all(&dir.join("templates"))?;
    Ok(dir)
}

pub fn get_templates_dir(app_name: &str) -> error::ConfigResult<PathBuf> {
    let dir = config_dir(app_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No config directory available"))?;
    Ok(dir.join("templates"))
}

pub fn load_templates_file(app_name: &str) -> error::ConfigResult<TemplatesConfig> {
    let dir = config_dir(app_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No config directory available"))?;
    let path = dir.join("templates.json");
    let file_content = fs::read_to_string(path)?;
    let templates_config: TemplatesConfig = serde_json::from_str(&file_content)?;
    Ok(templates_config)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplatesConfig {
    pub templates: Vec<Template>,
}

impl TemplatesConfig {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }

    pub fn write_to_file(&self, app_name: &str) -> error::ConfigResult<()> {
        let dir = ensure_config_dir(app_name)?;
        let path = dir.join("templates.json");
        let data = serde_json::to_string_pretty(self)?;
        fs::write(&path, data)?;
        Ok(())
    }
}
