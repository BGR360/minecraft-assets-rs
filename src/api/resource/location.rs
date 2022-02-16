use super::{ModelIdentifier, ResourceIdentifier, ResourceKind};

/// Represents a Minecraft [resource location].
///
/// Resource locations are namespaced identifiers referencing blocks, items,
/// entity types, recipes, functions, advancements, tags, and various other
/// objects in vanilla Minecraft.
///
/// To understand why this type has a lifetime parameter, see the
/// [`ResourceIdentifier`] documentation.
///
/// [resource location]: <https://minecraft.fandom.com/wiki/Resource_location>
/// [`ResourceIdentifier`]: ResourceIdentifier#borrowing--ownership
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceLocation<'a> {
    pub(crate) id: ResourceIdentifier<'a>,
    pub(crate) kind: ResourceKind,
}

impl<'a> ResourceLocation<'a> {
    /// Constructs a new [`ResourceLocation`] from the given type and id.
    pub fn new(kind: ResourceKind, id: impl Into<ResourceIdentifier<'a>>) -> Self {
        Self {
            id: id.into(),
            kind,
        }
    }

    /// Constructs a new [`ResourceLocation`] referencing the [`BlockStates`] of
    /// the given block id.
    ///
    /// [`BlockStates`]: ResourceKind::BlockStates
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let location = ResourceLocation::blockstates("stone");
    /// let location = ResourceLocation::blockstates("minecraft:dirt");
    /// ```
    pub fn blockstates(block_id: impl Into<ResourceIdentifier<'a>>) -> Self {
        Self::new(ResourceKind::BlockStates, block_id)
    }

    /// Constructs a new [`ResourceLocation`] referencing the [`BlockModel`] of
    /// the given block id.
    ///
    /// [`BlockModel`]: ResourceKind::BlockModel
    pub fn block_model(block_id: impl Into<ResourceIdentifier<'a>>) -> Self {
        Self::new(ResourceKind::BlockModel, block_id)
    }

    /// Constructs a new [`ResourceLocation`] referencing the [`ItemModel`] of
    /// the given item id.
    ///
    /// [`ItemModel`]: ResourceKind::ItemModel
    pub fn item_model(item_id: impl Into<ResourceIdentifier<'a>>) -> Self {
        Self::new(ResourceKind::ItemModel, item_id)
    }

    /// Constructs a new [`ResourceLocation`] referencing the [`Texture`]
    /// located at the given path.
    ///
    /// [`Texture`]: ResourceKind::Texture
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let location = ResourceLocation::texture("block/stone");
    /// let location = ResourceLocation::texture("item/diamond_hoe");
    pub fn texture(path: impl Into<ResourceIdentifier<'a>>) -> Self {
        Self::new(ResourceKind::Texture, path)
    }

    /// Returns the namespace of the resource referenced by this location.
    ///
    /// Analagous to [`ResourceIdentifier::namespace()`].
    pub fn namespace(&self) -> &str {
        self.id.namespace()
    }

    /// Returns the name / terminating "path" of the resource referenced by this
    /// location.
    ///
    /// Analagous to [`ResourceIdentifier::path()`].
    ///
    /// For [`BlockModel`] or [`ItemModel`] resources, the name will **not**
    /// include any leading prefix like `block/` or `item/`. See the
    /// [`ModelIdentifier`] documentation for more information.
    ///
    /// [`BlockModel`]: ResourceKind::BlockModel
    /// [`ItemModel`]: ResourceKind::ItemModel
    pub fn path(&self) -> &str {
        if self.is_model() {
            ModelIdentifier::model_name(self.id.path())
        } else {
            self.id.path()
        }
    }

    /// Returns what kind of resource is referenced by this location.
    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Returns true if the resource location refers to a built-in resource.
    ///
    /// If `true`, then there is no corresponding file that contains the
    /// resource.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let loc = ResourceLocation::new(ResourceKind::ItemModel, "builtin/generated");
    /// assert!(loc.is_builtin());
    /// ```
    pub fn is_builtin(&self) -> bool {
        if self.is_model() {
            ModelIdentifier::is_builtin(self.id.path())
        } else {
            false
        }
    }

    pub fn to_owned(&self) -> ResourceLocation<'static> {
        ResourceLocation {
            id: self.id.to_owned(),
            kind: self.kind,
        }
    }

    pub(crate) fn is_model(&self) -> bool {
        matches!(
            self.kind,
            ResourceKind::BlockModel | ResourceKind::ItemModel
        )
    }
}
