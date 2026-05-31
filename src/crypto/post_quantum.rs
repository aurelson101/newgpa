use anyhow::{bail, Result};

use crate::config::Settings;

#[derive(Debug, Clone, Copy)]
pub enum LabAlgorithm {
    MlKem,
    MlDsa,
    SlhDsa,
}

pub fn ensure_enabled(settings: &Settings) -> Result<()> {
    if !settings.post_quantum_lab_enabled {
        bail!("Post-Quantum Lab is experimental, non-standard OpenPGP, and disabled by default");
    }
    Ok(())
}

pub fn warning() -> &'static str {
    "Experimental feature: not compatible with standard OpenPGP. Do not use alone in production without cryptographic validation."
}
