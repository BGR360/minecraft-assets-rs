use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use crate::{
    api::{resource_location::ModelIdentifier, ResourceIdentifier, ResourceLocation, Result},
    schemas::{BlockStates, Model},
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
    ///
    /// # Example
    ///
    /// ```no_run
    /// use minecraft_assets::api::AssetPack;
    /// use minecraft_assets::schemas::BlockStates;
    ///
    /// let assets = AssetPack::at_path("~/.minecraft/");
    ///
    /// // Load the block states for `oak_planks`
    /// let states = assets.load_blockstates("oak_planks").unwrap();
    /// let variants = states.variants().unwrap();
    ///
    /// assert_eq!(variants.len(), 1);
    ///
    /// let model = &variants[""].models()[0];
    /// assert_eq!(model.model, "block/oak_planks");
    /// ```
    pub fn at_path(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root: PathBuf::from(root_dir.as_ref()),
        }
    }

    /// Returns the full path to a resource given a [`ResourceLocation`].
    ///
    /// **NOTE:** no validation of the path is performed. The returned path may
    /// not point to an existing file. This method simply computes what the path
    /// should be for a given resource.
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

    /// Loads the [`BlockStates`] of the block with the provided id.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let states = assets.load_blockstates("stone");
    /// let states = assets.load_blockstates("minecraft:dirt");
    /// ```
    pub fn load_blockstates<'a>(
        &self,
        block_id: impl Into<ResourceIdentifier<'a>>,
    ) -> Result<BlockStates> {
        self.load_resource(&ResourceLocation::BlockStates(block_id.into()))
    }

    /// Loads the block [`Model`] identified by the given name or path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let model = assets.load_block_model("stone");
    /// let model = assets.load_block_model("block/dirt");
    /// ```
    pub fn load_block_model<'a>(&self, model: impl Into<ModelIdentifier<'a>>) -> Result<Model> {
        self.load_resource(&ResourceLocation::BlockModel(model.into()))
    }

    /// Loads the block [`Model`] identified by the given name or path, as well
    /// as all of its parents and ancestors.
    ///
    /// The models are returned as a list, with the first element being the
    /// model that was originally requested, the next element being its parent,
    /// and so on with the last element being the topmost parent.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let models = assets.load_block_model_recursive("block/cube_all").unwrap();
    ///
    /// let expected = vec![
    ///     assets.load_block_model("block/cube_all").unwrap(),
    ///     assets.load_block_model("block/cube").unwrap(),
    ///     assets.load_block_model("block/block").unwrap(),
    /// ];
    /// assert_eq!(models, expected);
    /// ```
    pub fn load_block_model_recursive<'a>(
        &self,
        model: impl Into<ModelIdentifier<'a>>,
    ) -> Result<Vec<Model>> {
        self.load_model_recursive(&ResourceLocation::BlockModel(model.into()))
    }

    /// Loads the item [`Model`] identified by the given name or path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let model = assets.load_item_model("compass");
    /// let model = assets.load_item_model("item/diamond_hoe");
    /// ```
    pub fn load_item_model<'a>(&self, model: impl Into<ModelIdentifier<'a>>) -> Result<Model> {
        self.load_resource(&ResourceLocation::ItemModel(model.into()))
    }

    /// Loads the item [`Model`] identified by the given name or path, as well
    /// as all of its parents and ancestors.
    ///
    /// The models are returned as a list, with the first element being the
    /// model that was originally requested, the next element being its parent,
    /// and so on with the last element being the topmost parent.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let models = assets.load_item_model_recursive("item/diamond_hoe").unwrap();
    ///
    /// let expected = vec![
    ///     assets.load_item_model("item/diamond_hoe").unwrap(),
    ///     assets.load_item_model("item/handheld").unwrap(),
    ///     assets.load_item_model("item/generated").unwrap(),
    /// ];
    /// assert_eq!(models, expected);
    /// ```
    pub fn load_item_model_recursive<'a>(
        &self,
        model: impl Into<ModelIdentifier<'a>>,
    ) -> Result<Vec<Model>> {
        self.load_model_recursive(&ResourceLocation::ItemModel(model.into()))
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

    fn load_model_recursive(&self, resource: &ResourceLocation) -> Result<Vec<Model>> {
        let mut models = Vec::new();

        self.for_each_parent(resource.clone(), |model| models.push(model))?;

        Ok(models)
    }

    fn for_each_parent<F>(&self, mut current: ResourceLocation, mut op: F) -> Result<()>
    where
        F: FnMut(Model),
    {
        loop {
            let model: Model = self.load_resource(&current)?;

            let parent_owned = model
                .parent
                .as_ref()
                .map(|parent| ModelIdentifier::from(ResourceIdentifier::from(parent).into_owned()));

            println!("HULLO: {:?}", &model);

            op(model);

            if let Some(parent) = parent_owned {
                current = match current {
                    ResourceLocation::BlockModel(_) => ResourceLocation::BlockModel(parent),
                    ResourceLocation::ItemModel(_) => ResourceLocation::ItemModel(parent),
                    _ => unreachable!(),
                };
            } else {
                break;
            }
        }

        Ok(())
    }
}
