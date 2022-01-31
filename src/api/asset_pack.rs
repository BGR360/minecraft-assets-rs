use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use crate::{
    api::{ResourceIdentifier, ResourceLocation, Result},
    schemas::blockstates::BlockStates,
};

/// A collection of Minecraft assets at a given file path.
#[derive(Clone)]
pub struct AssetPack {
    /// Path to the directory that **contains** the `assets
    root: PathBuf,
}

impl AssetPack {
    /// Returns a new [`AssetPack`] that can read data from the given directory.
    ///
    /// The provided `root_dir` should be the directory that contains the
    /// `assets/` and/or `data/` directories.
    pub fn at_path(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root: PathBuf::from(root_dir.as_ref()),
        }
    }

    /// Returns the full path to a resource given a [`ResourceLocation`].
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let assets = AssetPack::at_path("~/.minecraft/");
    ///
    /// let loc = ResourceLocation::BlockStates("stone".into());
    /// assert_eq!(
    ///     assets.get_resource_path(&loc).to_string_lossy(),
    ///     "~/.minecraft/assets/minecraft/blockstates/stone.json"
    /// );
    /// ```
    pub fn get_resource_path(&self, resource: &ResourceLocation) -> PathBuf {
        let mut path = self.root.clone();
        path.push(&resource.path());
        path
    }

    /// Loads the [`BlockStates`] of the block with the provided [`ResourceIdentifier`].
    pub fn load_blockstates(&self, block: ResourceIdentifier) -> Result<BlockStates> {
        self.load_resource(&ResourceLocation::BlockStates(block))
    }

    fn load_resource<T>(&self, resource: &ResourceLocation) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = self.get_resource_path(resource);
        let file = fs::File::open(path)?;
        let resource: T = serde_json::from_reader(file)?;
        Ok(resource)
    }
}
