use thiserror::Error as TError;

pub use crate::{ImageError, SecCfgError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(TError, Debug)]
pub enum Error {
    #[error("Image parsing error: {0:?}")]
    Image(#[from] ImageError),

    #[error("Seccfg parsing error: {0:?}")]
    SecCfg(#[from] SecCfgError),

    #[error("Zerocopy error")]
    Zerocopy,
}
