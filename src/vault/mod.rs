use anyhow::{bail, Result};
use std::path::Path;

pub fn create_vault(_path: &Path) -> Result<()> {
    bail!(
        "secure vault is planned; initial implementation does not create encrypted containers yet"
    )
}
