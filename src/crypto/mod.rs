pub mod gpgme;
pub mod openpgp;
pub mod post_quantum;
pub mod smime;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("operation is disabled by high-security policy")]
    DisabledByPolicy,
    #[error("post-quantum lab is experimental and disabled by default")]
    PostQuantumDisabled,
}
