use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub high_security: bool,
    pub network_enabled: bool,
    pub post_quantum_lab_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            high_security: true,
            network_enabled: false,
            post_quantum_lab_enabled: false,
        }
    }
}

pub fn config_dir() -> Result<PathBuf> {
    let dirs =
        ProjectDirs::from("org", "NewGPA", "NewGPA").context("cannot resolve config directory")?;
    Ok(dirs.config_dir().to_path_buf())
}

pub fn load() -> Result<Settings> {
    let path = config_dir()?.join("settings.json");
    if !path.exists() {
        return Ok(Settings::default());
    }
    let bytes = fs::read(&path).with_context(|| format!("cannot read {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("cannot parse {}", path.display()))
}

pub fn save(settings: &Settings) -> Result<()> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    }
    let path = dir.join("settings.json");
    fs::write(&path, serde_json::to_vec_pretty(settings)?)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}
