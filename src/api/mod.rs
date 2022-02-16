//! An API for programmatically accessing Minecraft asset data.
//!
//! Start here: [`AssetPack`].

use std::io;

mod asset_pack;
mod resolve;
mod resource;

pub use asset_pack::AssetPack;
pub use resolve::ModelResolver;
pub use resource::{
    ModelIdentifier, ResourceCategory, ResourceKind, ResourceLocation, ResourcePath,
    MINECRAFT_NAMESPACE,
};

/// Error types that can be returned from API methods.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
}

/// Result alias for convenience.
pub type Result<T, E = Error> = std::result::Result<T, E>;
