use anyhow::Result;

use crate::crypto::gpgme::context_for_smime;

pub fn is_available() -> Result<bool> {
    let _ctx = context_for_smime()?;
    Ok(true)
}
