use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use crate::{
    api::{
        Error, ModelIdentifier, ResourceIdentifier, ResourceKind, ResourceLocation, ResourcePath,
        Result,
    },
    schemas::{BlockStates, Model},
};

/// A struct that can read Minecraft assets from a single root directory.
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
    ///
    /// let assets = AssetPack::at_path("~/.minecraft/");
    ///
    /// // Load the block states for `oak_planks`
    /// let states = assets.load_blockstates("oak_planks").unwrap();
    /// let variants = states.variants().unwrap();
    ///
    /// assert_eq!(variants.len(), 1);
    ///
    /// let model_properties = &variants[""].models()[0];
    /// assert_eq!(model_properties.model, "block/oak_planks");
    /// ```
    pub fn at_path(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root: PathBuf::from(root_dir.as_ref()),
        }
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
        self.load_resource(&ResourceLocation::blockstates(block_id))
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
    pub fn load_block_model<'a>(&self, model: impl Into<ResourceIdentifier<'a>>) -> Result<Model> {
        self.load_resource(&ResourceLocation::block_model(model))
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
        model: impl Into<ResourceIdentifier<'a>>,
    ) -> Result<Vec<Model>> {
        self.load_model_recursive(&ResourceLocation::block_model(model))
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
    pub fn load_item_model<'a>(&self, model: impl Into<ResourceIdentifier<'a>>) -> Result<Model> {
        self.load_resource(&ResourceLocation::item_model(model))
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
        model: impl Into<ResourceIdentifier<'a>>,
    ) -> Result<Vec<Model>> {
        self.load_model_recursive(&ResourceLocation::item_model(model.into()))
    }

    /// Runs the given closure once for each file that exists in
    /// `assets/<namespace>/blockstates/`.
    ///
    /// The closure is passed the full path to each file.
    pub fn for_each_blockstates<F, E>(&self, namespace: &str, op: F) -> Result<()>
    where
        F: FnMut(&ResourceIdentifier, &Path) -> Result<(), E>,
        Error: From<E>,
    {
        self.for_each_file(namespace, ResourceKind::BlockStates, op)
    }

    /// Runs the given closure once for each file that exists in
    /// `assets/<namespace>/models/block/`.
    ///
    /// The closure is passed the full path to each file.
    pub fn for_each_block_model<F, E>(&self, namespace: &str, op: F) -> Result<()>
    where
        F: FnMut(&ResourceIdentifier, &Path) -> Result<(), E>,
        Error: From<E>,
    {
        self.for_each_file(namespace, ResourceKind::BlockModel, op)
    }

    /// Runs the given closure once for each file that exists in
    /// `assets/<namespace>/models/item/`.
    ///
    /// The closure is passed the full path to each file.
    pub fn for_each_item_model<F, E>(&self, namespace: &str, op: F) -> Result<()>
    where
        F: FnMut(&ResourceIdentifier, &Path) -> Result<(), E>,
        Error: From<E>,
    {
        self.for_each_file(namespace, ResourceKind::ItemModel, op)
    }

    /// Runs the given closure once for each file that exists in
    /// `assets/<namespace>/textures/`.
    ///
    /// The closure is passed the [`ResourceIdentifier`] for each texture as
    /// well as the full path to each image file.
    pub fn for_each_texture<F, E>(&self, namespace: &str, op: F) -> Result<()>
    where
        F: FnMut(&ResourceIdentifier, &Path) -> Result<(), E>,
        Error: From<E>,
    {
        self.for_each_file(namespace, ResourceKind::Texture, op)
    }

    /// Loads a given resource directly given the full path to its file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # use minecraft_assets::schemas::BlockStates;
    /// # let assets = AssetPack::at_path("foo");
    /// let blockstates: BlockStates = assets.load_resource_at_path(
    ///     "~/.minecraft/assets/minecraft/blockstates/stone.json"
    /// ).unwrap();
    /// ```
    pub fn load_resource_at_path<T>(&self, path: impl AsRef<Path>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let file = fs::File::open(path)?;
        let resource: T = serde_json::from_reader(file)?;
        Ok(resource)
    }

    fn load_resource<T>(&self, resource: &ResourceLocation) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let path = ResourcePath::for_resource(&self.root, resource);
        self.load_resource_at_path(path)
    }

    fn load_model_recursive(&self, resource: &ResourceLocation) -> Result<Vec<Model>> {
        let mut models = Vec::new();

        Self::for_each_parent(
            resource.clone(),
            |model| models.push(model),
            |next_location| self.load_resource(next_location),
        )?;

        Ok(models)
    }

    pub(crate) fn for_each_parent<F, L, E>(
        mut current: ResourceLocation,
        mut op: F,
        mut load_model: L,
    ) -> Result<(), E>
    where
        F: FnMut(Model),
        L: FnMut(&ResourceLocation) -> Result<Model, E>,
    {
        loop {
            let model = load_model(&current)?;

            let parent_owned = model
                .parent
                .as_ref()
                .map(|parent| ResourceIdentifier::from(parent.as_str()).to_owned());

            op(model);

            match parent_owned {
                Some(parent) if !ModelIdentifier::is_builtin(parent.path()) => {
                    //println!("{}", parent.as_str());
                    current = match current.kind {
                        ResourceKind::BlockModel => {
                            ResourceLocation::block_model(parent.as_str()).to_owned()
                        }
                        ResourceKind::ItemModel => {
                            ResourceLocation::item_model(parent.as_str()).to_owned()
                        }
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }

        Ok(())
    }

    fn for_each_file<F, E>(&self, namespace: &str, kind: ResourceKind, mut op: F) -> Result<()>
    where
        F: FnMut(&ResourceIdentifier, &Path) -> Result<(), E>,
        Error: From<E>,
    {
        let directory = ResourcePath::for_kind(&self.root, namespace, kind);

        self.for_each_file_inner(&directory, &directory, &mut op)
    }

    fn for_each_file_inner<F, E>(
        &self,
        original_directory: &Path,
        current_directory: &Path,
        op: &mut F,
    ) -> Result<()>
    where
        F: FnMut(&ResourceIdentifier, &Path) -> Result<(), E>,
        Error: From<E>,
    {
        for entry in fs::read_dir(current_directory)? {
            let entry = entry?;

            let entry_path = entry.path();

            if entry_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with('_')
            {
                continue;
            }

            if entry.file_type()?.is_dir() {
                self.for_each_file_inner(original_directory, &entry_path, op)?;
            } else {
                let suffix = entry_path.strip_prefix(original_directory).unwrap();
                let suffix = suffix.with_extension("");
                let suffix = suffix.to_string_lossy();

                op(&ResourceIdentifier::from(suffix.as_ref()), &entry_path)?;
            }
        }

        Ok(())
    }
}
