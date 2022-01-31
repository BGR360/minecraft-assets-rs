use std::path::PathBuf;

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
/// [resource location]: <https://minecraft.fandom.com/wiki/Resource_location>
pub enum ResourceLocation {
    /// Represents the location of a file in `assets/<namespace>/blockstates/`.
    BlockStates(ResourceIdentifier),
}

impl ResourceLocation {
    /// Returns a reference to the underlying [`ResourceIdentifier`].
    pub(crate) fn id(&self) -> &ResourceIdentifier {
        match self {
            Self::BlockStates(ref id) => id,
        }
    }

    /// Returns the type of resource that this location references.
    pub fn kind(&self) -> ResourceKind {
        match self {
            Self::BlockStates(_) => ResourceKind::Assets,
        }
    }

    /// Returns the namespace of the resource referenced by this location.
    pub fn namespace(&self) -> &str {
        self.id().namespace()
    }

    /// Returns the name / terminating "path" of the resource referenced by this location.
    pub fn name(&self) -> &str {
        self.id().path()
    }

    /// Returns the path relative to `{assets,data}/<namespace>/` at which the
    /// resource's file can be found.
    pub fn directory(&self) -> &'static str {
        match self {
            Self::BlockStates(_) => "blockstates",
        }
    }

    /// Returns the file extension (e.g., `json`) used for this resource's file.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::BlockStates(_) => "json",
        }
    }

    /// Returns a file path relative to the asset root at which the resource can
    /// be found.
    pub fn path(&self) -> PathBuf {
        let mut path = match self.kind() {
            ResourceKind::Assets => PathBuf::from("assets"),
            ResourceKind::Data => PathBuf::from("data"),
        };
        path.push(self.namespace());
        path.push(self.directory());
        path.push(self.name());

        path.with_extension(self.extension())
    }
}

/// A namespaced identifier for an undetermined type of resource.
///
/// A valid resource location has a format of `namespace:path`. If the
/// `namespace` portion is left out, then `minecraft` is the implied namespace.
pub struct ResourceIdentifier(String);

impl ResourceIdentifier {
    /// Returns whether or not this resource location includes an explicity
    /// namespace.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::ResourceIdentifier;
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
    /// # use minecraft_assets::api::ResourceIdentifier;
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
    /// # use minecraft_assets::api::ResourceIdentifier;
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

    fn colon_position(&self) -> Option<usize> {
        self.0.chars().position(|c| c == ':')
    }
}

impl<S: Into<String>> From<S> for ResourceIdentifier {
    fn from(source: S) -> Self {
        Self(source.into())
    }
}

impl AsRef<str> for ResourceIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
