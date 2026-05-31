use anyhow::{Context, Result};
use std::{fs::File, path::Path};

use crate::crypto::gpgme::context_for_openpgp;

#[derive(Debug, Clone)]
pub struct EncryptOptions {
    pub armor: bool,
    pub always_trust: bool,
}

impl Default for EncryptOptions {
    fn default() -> Self {
        Self {
            armor: true,
            always_trust: false,
        }
    }
}

pub fn encrypt_file(
    input: &Path,
    output: &Path,
    recipient_fingerprints: &[String],
    options: EncryptOptions,
) -> Result<()> {
    let mut ctx = context_for_openpgp()?;
    ctx.set_armor(options.armor);
    let keys = recipient_fingerprints
        .iter()
        .map(|fpr| {
            ctx.get_key(fpr)
                .with_context(|| format!("recipient key not found: {fpr}"))
        })
        .collect::<Result<Vec<_>>>()?;
    let mut source =
        File::open(input).with_context(|| format!("cannot open {}", input.display()))?;
    let mut sink =
        File::create(output).with_context(|| format!("cannot create {}", output.display()))?;
    let flags = if options.always_trust {
        gpgme::EncryptFlags::ALWAYS_TRUST
    } else {
        gpgme::EncryptFlags::empty()
    };
    ctx.encrypt_with_flags(&keys, &mut source, &mut sink, flags)?;
    Ok(())
}

pub fn decrypt_file(input: &Path, output: &Path) -> Result<()> {
    let mut ctx = context_for_openpgp()?;
    let mut source =
        File::open(input).with_context(|| format!("cannot open {}", input.display()))?;
    let mut sink =
        File::create(output).with_context(|| format!("cannot create {}", output.display()))?;
    ctx.decrypt(&mut source, &mut sink)?;
    Ok(())
}
