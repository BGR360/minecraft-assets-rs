use std::{borrow::Cow, fmt};

#[allow(missing_docs)]
pub const MINECRAFT_NAMESPACE: &str = "minecraft";

use crate::api::{ModelIdentifier, ResourceKind};

/// Represents a Minecraft [resource location].
///
/// Resource locations are namespaced identifiers referencing blocks, items,
/// entity types, recipes, functions, advancements, tags, and various other
/// objects in vanilla Minecraft.
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
/// [`to_owned()`][Self::to_owned] to get an owned identifier.
///
/// [resource location]: <https://minecraft.fandom.com/wiki/Resource_location>
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResourceLocation<'a> {
    pub(crate) id: Cow<'a, str>,
    pub(crate) kind: ResourceKind,
}

impl<'a> ResourceLocation<'a> {
    /// Constructs a new [`ResourceLocation`] from the given type and id.
    ///
    /// The `id` string will be **borrowed**. You can either use [`to_owned()`]
    /// to convert the location to an owned representation, or construct on
    /// directly using [`new_owned()`].
    ///
    /// [`to_owned()`]: Self::to_owned
    /// [`new_owned()`]: Self::new_owned
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let location = ResourceLocation::new(ResourceKind::BlockModel, "oak_stairs");
    /// ```
    pub fn new(kind: ResourceKind, id: &'a str) -> Self {
        Self {
            id: Cow::Borrowed(id),
            kind,
        }
    }

    /// Like [`new()`], but returns a [`ResourceLocation`] that owns its
    /// internal string.
    ///
    /// [`new()`]: Self::new
    pub fn new_owned(kind: ResourceKind, id: String) -> ResourceLocation<'static> {
        ResourceLocation {
            id: Cow::Owned(id),
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
    pub fn blockstates(block_id: &'a str) -> Self {
        Self::new(ResourceKind::BlockStates, block_id)
    }

    /// Constructs a new [`ResourceLocation`] referencing the [`BlockModel`] of
    /// the given block id.
    ///
    /// [`BlockModel`]: ResourceKind::BlockModel
    pub fn block_model(block_id: &'a str) -> Self {
        Self::new(ResourceKind::BlockModel, block_id)
    }

    /// Constructs a new [`ResourceLocation`] referencing the [`ItemModel`] of
    /// the given item id.
    ///
    /// [`ItemModel`]: ResourceKind::ItemModel
    pub fn item_model(item_id: &'a str) -> Self {
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
    pub fn texture(path: &'a str) -> Self {
        Self::new(ResourceKind::Texture, path)
    }

    /// Returns the underlying identifier as a string slice.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let location = ResourceLocation::blockstates("stone");
    /// assert_eq!(location.as_str(), "stone");
    ///
    /// let location = ResourceLocation::blockstates("minecraft:dirt");
    /// assert_eq!(location.as_str(), "minecraft:dirt");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.id
    }

    /// Returns whether or not this resource location includes an explicit
    /// namespace.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let location = ResourceLocation::blockstates("foo:bar");
    /// assert!(location.has_namespace());
    ///
    /// let location = ResourceLocation::blockstates("bar");
    /// assert!(!location.has_namespace());
    /// ```
    pub fn has_namespace(&self) -> bool {
        self.colon_position().is_some()
    }

    /// Returns the namespace portion of the resource identifier, or
    /// `"minecraft"` if it does not have an explicit namespace.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let location = ResourceLocation::blockstates("foo:bar");
    /// assert_eq!(location.namespace(), "foo");
    ///
    /// let location = ResourceLocation::blockstates("bar");
    /// assert_eq!(location.namespace(), "minecraft");
    ///
    /// let location = ResourceLocation::blockstates(":bar");
    /// assert_eq!(location.namespace(), "");
    /// ```
    pub fn namespace(&self) -> &str {
        self.colon_position()
            .map(|index| &self.id[..index])
            .unwrap_or_else(|| MINECRAFT_NAMESPACE)
    }

    /// Returns the path portion of the resource location.
    ///
    /// # Note on Models
    ///
    /// For [`BlockModel`] or [`ItemModel`] resources, the name will **not**
    /// include any leading prefix like `block/` or `item/`. See the
    /// [`ModelIdentifier`] documentation for more information.
    ///
    /// [`BlockModel`]: ResourceKind::BlockModel
    /// [`ItemModel`]: ResourceKind::ItemModel
    pub fn path(&self) -> &str {
        if self.is_model() {
            ModelIdentifier::model_name(&self.id)
        } else {
            &self.id
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
    /// let loc = ResourceLocation::item_model("builtin/generated");
    /// assert!(loc.is_builtin());
    /// ```
    pub fn is_builtin(&self) -> bool {
        if self.is_model() {
            ModelIdentifier::is_builtin(&self.id)
        } else {
            false
        }
    }

    /// Returns a new location with a canonical representation (i.e.,
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
    /// let location = ResourceLocation::blockstates("stone");
    /// let canonical = location.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "minecraft:stone");
    /// ```
    ///
    /// Performs a shallow copy when a namespace is already present:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let location = ResourceLocation::blockstates("foo:bar");
    /// let canonical = location.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "foo:bar");
    ///
    /// // Prove that it was a cheap copy.
    /// assert_eq!(
    ///     location.as_str().as_ptr() as usize,
    ///     canonical.as_str().as_ptr() as usize,
    /// );
    /// ```
    pub fn to_canonical(&self) -> ResourceLocation<'a> {
        if self.has_namespace() {
            self.clone()
        } else {
            let canonical = format!("{}:{}", self.namespace(), self.as_str());
            Self {
                id: Cow::Owned(canonical),
                kind: self.kind,
            }
        }
    }

    /// Returns a new [`ResourceLocation`] that owns the underlying string.
    ///
    /// This is useful for, e.g., storing the location in a data structure or
    /// passing it to another thread.
    ///
    /// By default, all `ResourceLocation`s borrow the string they are
    /// constructed with, so no copying will occur unless you call this
    /// function.
    ///
    /// # Examples
    ///
    /// Constructing a location using [`From`] simply borrows the data:
    ///
    /// ```compile_fail
    /// # use minecraft_assets::api::*;
    /// let string = String::new("my:resource");
    ///
    /// let location = ResourceLocation::from(&string);
    ///
    /// // Location borrows data from `string`, cannot be sent across threads.
    /// std::thread::spawn(move || println!("{}", location));
    /// ```
    ///
    /// Calling [`to_owned()`][Self::to_owned] on the location allows it to be
    /// sent to the thread:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let string = "my:resource".to_string();
    ///
    /// let location = ResourceLocation::blockstates(&string);
    /// let location = location.to_owned();
    ///
    /// std::thread::spawn(move || println!("{}", location));
    /// ```
    pub fn to_owned(&self) -> ResourceLocation<'static> {
        ResourceLocation {
            id: Cow::Owned(self.id.clone().into_owned()),
            kind: self.kind,
        }
    }

    pub(crate) fn is_model(&self) -> bool {
        matches!(
            self.kind,
            ResourceKind::BlockModel | ResourceKind::ItemModel
        )
    }

    fn colon_position(&self) -> Option<usize> {
        self.id.chars().position(|c| c == ':')
    }
}

impl<'a> AsRef<str> for ResourceLocation<'a> {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

impl<'a> fmt::Debug for ResourceLocation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = format!("{:?}", self.kind);
        write!(f, "{}({:?})", kind, &self.id)
    }
}

impl<'a> fmt::Display for ResourceLocation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical().as_str())
    }
}
