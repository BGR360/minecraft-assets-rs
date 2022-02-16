use std::{borrow::Cow, fmt, ops::Deref};

pub const MINECRAFT_NAMESPACE: &str = "minecraft";

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
/// [`to_owned()`][Self::to_owned] to get an owned identifier.
#[derive(Clone, Hash)]
pub struct ResourceIdentifier<'a>(Cow<'a, str>);

impl<'a> ResourceIdentifier<'a> {
    /// Returns this identifier's underlying string representation.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::from("stone");
    /// assert_eq!(id.as_str(), "stone");
    ///
    /// let id = ResourceIdentifier::from("minecraft:dirt");
    /// assert_eq!(id.as_str(), "minecraft:dirt");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether or not this resource identifier includes an explicit
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

    /// Returns the namespace portion of the resource identifier, or
    /// `"minecraft"` if it does not have an explicit namespace.
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

    /// Returns the path portion of the resource identifier.
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
    /// let id = ResourceIdentifier::from("stone");
    /// let canonical = id.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "minecraft:stone");
    /// ```
    ///
    /// Performs a shallow copy when a namespace is already present:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::from("foo:bar");
    /// let canonical = id.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "foo:bar");
    ///
    /// // Prove that it was a cheap copy.
    /// assert_eq!(
    ///     id.as_str().as_ptr() as usize,
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
    /// let id = ResourceIdentifier::from(&string);
    ///
    /// // Identifier borrows data from `string`, cannot be sent across threads.
    /// std::thread::spawn(move || println!("{}", id));
    /// ```
    ///
    /// Calling [`to_owned()`][Self::to_owned] on the identifier allows it
    /// to be sent to the thread:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let string = String::from("my:ident");
    ///
    /// let id = ResourceIdentifier::from(&string);
    /// let id = id.to_owned();
    ///
    /// std::thread::spawn(move || println!("{}", id));
    /// ```
    pub fn to_owned(&self) -> ResourceIdentifier<'static> {
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

/// A wrapper around a namespaced identifier for a block or item model.
///
/// # Why does this exist?
///
/// Prior to 1.13, model identifiers found in
/// `assets/<namespace>/blockstates/*.json` did not include a prefix like
/// `block/` or `item/` to disambiguate between different types of models.
///
/// Because of this, the `minecraft-assets` API forces the user to always
/// specify which type of model they are trying to reference (note the existence
/// of both [`BlockModel`] and [`ItemModel`] variants in [`ResourceKind`]). This
/// way, the API will work with versions prior to 1.13.
///
/// So this struct is meant to wrap an identifier and extract its model name.
/// See the [`model_name()`] documentation for more information.
///
/// [`ResourceKind`]: crate::api::ResourceKind
/// [`BlockModel`]: crate::api::ResourceKind::BlockModel
/// [`ItemModel`]: crate::api::ResourceKind::ItemModel
/// [`model_name()`]: Self::model_name
#[derive(Debug, Clone, Hash)]
pub struct ModelIdentifier;

impl ModelIdentifier {
    /// Returns the name of the model, stripping the leading path component if
    /// there is one.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// assert_eq!(ModelIdentifier::model_name("stone"), "stone");
    /// assert_eq!(ModelIdentifier::model_name("block/oak_planks"), "oak_planks");
    /// assert_eq!(ModelIdentifier::model_name("item/diamond_hoe"), "diamond_hoe");
    /// ```
    pub fn model_name(id: &str) -> &str {
        Self::slash_position(id)
            .map(|index| &id[index + 1..])
            .unwrap_or_else(|| id)
    }

    pub(crate) fn is_builtin(id: &str) -> bool {
        match Self::slash_position(id) {
            Some(index) => {
                let prefix = &id[..index];
                prefix == "builtin"
            }
            None => false,
        }
    }

    fn slash_position(id: &str) -> Option<usize> {
        id.chars().position(|c| c == '/')
    }
}
