use std::{borrow::Cow, fmt, ops::Deref, path::PathBuf};

pub const MINECRAFT_NAMESPACE: &str = "minecraft";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Resources located in the `assets/` directory.
    Assets,

    /// Resource located in the `data/` directory.
    Data,
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceLocation<'a> {
    /// Represents the location of a file in `assets/<namespace>/blockstates/`.
    BlockStates(ResourceIdentifier<'a>),

    /// Represents the location of a file in `assets/<namespace>/models/block/`.
    BlockModel(ModelIdentifier<'a>),

    /// Represents the location of a file in `assets/<namespace>/models/item/`.
    ItemModel(ModelIdentifier<'a>),
}

impl<'a> ResourceLocation<'a> {
    /// Returns a reference to the underlying [`ResourceIdentifier`].
    pub(crate) fn id(&self) -> &ResourceIdentifier<'a> {
        match self {
            Self::BlockModel(id) | Self::ItemModel(id) => id.deref(),
            Self::BlockStates(id) => id,
        }
    }

    /// Returns the type of resource that this location references.
    pub fn kind(&self) -> ResourceKind {
        match self {
            Self::BlockStates(_) | Self::BlockModel(_) | Self::ItemModel(_) => ResourceKind::Assets,
        }
    }

    /// Returns the namespace of the resource referenced by this location.
    pub fn namespace(&self) -> &str {
        self.id().namespace()
    }

    /// Returns the name / terminating "path" of the resource referenced by this
    /// location.
    ///
    /// For [`BlockModel`] or [`ItemModel`] variants, the name will **not**
    /// include any leading prefix like `block/` or `item/`. See the
    /// [`ModelIdentifier`] documentation for more information.
    ///
    /// [`BlockModel`]: Self::BlockModel
    /// [`ItemModel`]: Self::ItemModel
    pub fn name(&self) -> &str {
        match self {
            Self::BlockModel(id) | Self::ItemModel(id) => id.model_name(),
            _ => self.id().path(),
        }
    }

    /// Returns the name of the model specified by a [`BlockModel`] or
    /// [`ItemModel`], or `None` if `self` is not one of those variants.
    ///
    /// See the [`ModelIdentifier`] documentation for more information.
    ///
    /// [`BlockModel`]: Self::BlockModel
    /// [`ItemModel`]: Self::ItemModel
    pub fn model_name(&self) -> Option<&str> {
        match self {
            Self::BlockModel(id) | Self::ItemModel(id) => Some(id.model_name()),

            _ => None,
        }
    }

    /// Returns the path to the directory that should contain this resource's file,
    /// relative to the [`AssetPack`] root.
    ///
    /// [`AssetPack`]: crate::api::AssetPack
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let loc = ResourceLocation::BlockStates("stone".into());
    /// assert_eq!(
    ///     loc.directory().to_string_lossy(),
    ///     "assets/minecraft/blockstates"
    /// );
    /// ```
    pub fn directory(&self) -> PathBuf {
        let mut path = match self.kind() {
            ResourceKind::Assets => PathBuf::from("assets"),
            ResourceKind::Data => PathBuf::from("data"),
        };
        path.push(self.namespace());

        let remaining = match self {
            Self::BlockStates(_) => "blockstates",
            Self::BlockModel(_) => "models/block",
            Self::ItemModel(_) => "models/item",
        };
        path.push(remaining);

        path
    }

    /// Returns the file extension (e.g., `json`) used for this resource's file.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::BlockStates(_) | Self::BlockModel(_) | Self::ItemModel(_) => "json",
        }
    }

    /// Returns the path to the file that should contain this resource, relative
    /// to the [`AssetPack`] root.
    ///
    /// [`AssetPack`]: crate::api::AssetPack
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let loc = ResourceLocation::BlockStates("stone".into());
    /// assert_eq!(
    ///     loc.path().to_string_lossy(),
    ///     "assets/minecraft/blockstates/stone.json"
    /// );
    /// ```
    pub fn path(&self) -> PathBuf {
        let mut path = self.directory();
        path.push(self.name());

        path.with_extension(self.extension())
    }
}

/// A namespaced identifier for an undetermined type of resource.
///
/// A valid resource location has a format of `"namespace:path"`. If the
/// `namespace` portion is left out, then `"minecraft"` is the implied
/// namespace.
///
/// # Borrowing / Ownership
///
/// To avoid cloning / [`String`] construction when not necessary, this type can
/// either borrow or take ownership of the underlying string.
///
/// By default, no copying or allocating is done. You must call
/// [`into_owned()`][Self::into_owned] to get an owned identifier.
#[derive(Clone)]
pub struct ResourceIdentifier<'a>(Cow<'a, str>);

impl<'a> ResourceIdentifier<'a> {
    /// Returns this identifier's underlying string representation.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let ident = ResourceIdentifier::from("stone");
    /// assert_eq!(ident.as_str(), "stone");
    ///
    /// let ident = ResourceIdentifier::from("minecraft:dirt");
    /// assert_eq!(ident.as_str(), "minecraft:dirt");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether or not this resource location includes an explicit
    /// namespace.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::from("foo:bar");
    /// assert!(id.has_namespace());
    ///
    /// let id = ResourceIdentifier::from("bar");
    /// assert!(!id.has_namespace());
    /// ```
    pub fn has_namespace(&self) -> bool {
        self.colon_position().is_some()
    }

    /// Returns the namespace portion of the resource location.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::from("foo:bar");
    /// assert_eq!(id.namespace(), "foo");
    ///
    /// let id = ResourceIdentifier::from("bar");
    /// assert_eq!(id.namespace(), "minecraft");
    ///
    /// let id = ResourceIdentifier::from(":bar");
    /// assert_eq!(id.namespace(), "");
    /// ```
    pub fn namespace(&self) -> &str {
        self.colon_position()
            .map(|index| &self.0[..index])
            .unwrap_or_else(|| MINECRAFT_NAMESPACE)
    }

    /// Returns the path portion of the resource location.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::from("foo:bar");
    /// assert_eq!(id.path(), "bar");
    ///
    /// let id = ResourceIdentifier::from("bar");
    /// assert_eq!(id.path(), "bar");
    ///
    /// let id = ResourceIdentifier::from("foo:");
    /// assert_eq!(id.path(), "");
    /// ```
    pub fn path(&self) -> &str {
        self.colon_position()
            .map(|index| &self.0[index + 1..])
            .unwrap_or_else(|| &self.0[..])
    }

    /// Returns a new identifier with a canonical representation (i.e.,
    /// containing an explicit namespace).
    ///
    /// This will involve allocating a new [`String`] if `self` does not already
    /// contain an explicit namespace.
    ///
    /// # Examples
    ///
    /// Prepends the default namespace when one is not present:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let ident = ResourceIdentifier::from("stone");
    /// let canonical = ident.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "minecraft:stone");
    /// ```
    ///
    /// Performs a shallow copy when a namespace is already present:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let ident = ResourceIdentifier::from("foo:bar");
    /// let canonical = ident.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "foo:bar");
    ///
    /// // Prove that it was a cheap copy.
    /// assert_eq!(
    ///     ident.as_str().as_ptr() as usize,
    ///     canonical.as_str().as_ptr() as usize,
    /// );
    /// ```
    pub fn to_canonical(&self) -> ResourceIdentifier<'a> {
        if self.has_namespace() {
            self.clone()
        } else {
            let canonical = format!("{}:{}", self.namespace(), self.as_str());
            ResourceIdentifier(Cow::Owned(canonical))
        }
    }

    /// Returns a new [`ResourceIdentifier`] that owns the underlying string.
    ///
    /// This is useful for, e.g., storing the identifier in a data structure or
    /// passing it to another thread.
    ///
    /// By default, all `ResourceIdentifier`s borrow the string they are
    /// constructed with, so no copying will occur unless you call this
    /// function.
    ///
    /// # Examples
    ///
    /// Constructing an identifier using [`From`] simply borrows the data:
    ///
    /// ```compile_fail
    /// # use minecraft_assets::api::*;
    /// let string = String::from("my:ident");
    ///
    /// let ident = ResourceIdentifier::from(&string);
    ///
    /// // Identifier borrows data from `string`, cannot be sent across threads.
    /// std::thread::spawn(move || println!("{}", ident));
    /// ```
    ///
    /// Calling [`into_owned()`][Self::into_owned] on the identifier allows it
    /// to be sent to the thread:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let string = String::from("my:ident");
    ///
    /// let ident = ResourceIdentifier::from(&string);
    /// let ident = ident.into_owned();
    ///
    /// std::thread::spawn(move || println!("{}", ident));
    /// ```
    pub fn into_owned(&self) -> ResourceIdentifier<'static> {
        let string = self.0.deref().to_owned();
        ResourceIdentifier(Cow::Owned(string))
    }

    fn colon_position(&self) -> Option<usize> {
        self.0.chars().position(|c| c == ':')
    }
}

impl<'a, S> From<&'a S> for ResourceIdentifier<'a>
where
    S: AsRef<str> + ?Sized,
{
    fn from(source: &'a S) -> Self {
        Self(Cow::Borrowed(source.as_ref()))
    }
}

impl<'a> PartialEq for ResourceIdentifier<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> Eq for ResourceIdentifier<'a> {}

impl<'a> fmt::Debug for ResourceIdentifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ResId").field(&self.as_str()).finish()
    }
}

impl<'a> fmt::Display for ResourceIdentifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical().as_str())
    }
}

/// A namespaced identifier for a block or item model.
///
/// Prior to 1.13, model identifiers found in
/// `assets/<namespace>/blockstates/*.json` did not include a prefix like
/// `block/` or `ident/` to disambiguate between different types of models.
///
/// Because of this, the `minecraft-assets` API forces the user to always
/// specify which type of model they are trying to load (note the existence of
/// both [`BlockModel`] and [`ItemModel`] variants in [`ResourceLocation`]).
/// This way, the API will work with versions prior to 1.13.
///
/// This struct exists mostly for internal convenience so we can scrap the
/// leading `block/` or `item/` if it is included.
///
/// [`BlockModel`]: ResourceLocation::BlockModel
/// [`ItemModel`]: ResourceLocation::ItemModel
#[derive(Debug, Clone)]
pub struct ModelIdentifier<'a>(ResourceIdentifier<'a>);

impl<'a> ModelIdentifier<'a> {
    /// Returns the name of the model, stripping the leading path component if
    /// there is one.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let ident = ModelIdentifier::from("stone");
    /// assert_eq!(ident.model_name(), "stone");
    /// let ident = ModelIdentifier::from("foo:stone");
    /// assert_eq!(ident.model_name(), "stone");
    ///
    /// let ident = ModelIdentifier::from("block/oak_planks");
    /// assert_eq!(ident.model_name(), "oak_planks");
    /// let ident = ModelIdentifier::from("foo:block/oak_planks");
    /// assert_eq!(ident.model_name(), "oak_planks");
    ///
    /// let ident = ModelIdentifier::from("item/diamond_hoe");
    /// assert_eq!(ident.model_name(), "diamond_hoe");
    /// let ident = ModelIdentifier::from("foo:item/diamond_hoe");
    /// assert_eq!(ident.model_name(), "diamond_hoe");
    ///
    /// ```
    pub fn model_name(&self) -> &str {
        self.slash_position()
            .map(|index| &self.0.path()[index + 1..])
            .unwrap_or_else(|| self.0.path())
    }

    fn slash_position(&self) -> Option<usize> {
        self.0.path().chars().position(|c| c == '/')
    }
}

impl<'a, S> From<S> for ModelIdentifier<'a>
where
    S: Into<ResourceIdentifier<'a>>,
{
    fn from(source: S) -> Self {
        Self(source.into())
    }
}

impl<'a> Deref for ModelIdentifier<'a> {
    type Target = ResourceIdentifier<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> PartialEq for ModelIdentifier<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.model_name() == other.model_name()
    }
}

impl<'a> Eq for ModelIdentifier<'a> {}
