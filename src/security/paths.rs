use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};

pub fn reject_path_traversal(path: &Utf8Path) -> Result<Utf8PathBuf> {
    if path
        .components()
        .any(|c| matches!(c, camino::Utf8Component::ParentDir))
    {
        bail!("path traversal is not allowed");
    }
    Ok(path.to_path_buf())
}
