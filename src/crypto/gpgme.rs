use anyhow::Result;

pub fn context_for_openpgp() -> Result<gpgme::Context> {
    let mut ctx = gpgme::Context::from_protocol(gpgme::Protocol::OpenPgp)?;
    ctx.set_armor(true);
    Ok(ctx)
}

pub fn context_for_smime() -> Result<gpgme::Context> {
    gpgme::Context::from_protocol(gpgme::Protocol::Cms).map_err(Into::into)
}
