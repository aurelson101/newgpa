use anyhow::Result;

use crate::crypto::gpgme::context_for_openpgp;

#[derive(Debug, Clone)]
pub struct KeySummary {
    pub fingerprint: String,
    pub user_id: String,
    pub expires_at: Option<String>,
    pub has_secret: bool,
}

pub fn list_public_keys() -> Result<Vec<KeySummary>> {
    let mut ctx = context_for_openpgp()?;
    let mut keys = Vec::new();
    for item in ctx.keys()? {
        let key = item?;
        let fingerprint = key.fingerprint().unwrap_or("unknown").to_string();
        let user_id = key
            .user_ids()
            .next()
            .and_then(|uid| uid.id().ok())
            .unwrap_or("unknown user id")
            .to_string();
        let expires_at = key
            .primary_key()
            .and_then(|subkey| subkey.expiration_time())
            .map(|time| format!("{time:?}"));
        keys.push(KeySummary {
            fingerprint,
            user_id,
            expires_at,
            has_secret: key.has_secret(),
        });
    }
    Ok(keys)
}
