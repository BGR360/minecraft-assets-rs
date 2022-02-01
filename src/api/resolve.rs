use crate::schemas::models::{Model, Textures};

/// Methods for resolving the properties of a [`Model`] with respect to its
/// parents.
pub struct ModelResolver;

impl ModelResolver {
    /// Iterates through a [`Model`] and all of its parents to resolve all of
    /// the model's [texture variables].
    ///
    /// This works by merging together the [`Textures`] maps from all models in
    /// the parent-child chain, and then substituting texture variables with
    /// concrete values where possible.
    ///
    /// [texture variables]: Textures#texture-variables
    ///
    /// # Example
    ///
    /// ```
    /// use maplit::hashmap;
    ///
    /// use minecraft_assets::api::{ModelResolver, ResourceLocation};
    /// use minecraft_assets::schemas::models::{Model, Textures};
    ///
    /// let child = Model {
    ///     parent: Some(String::from("parent")),
    ///     textures: Some(Textures::from(hashmap! {
    ///         "child_texture" => "textures/child",
    ///         "bar" => "#parent_texture"
    ///     })),
    ///     ..Default::default()
    /// };
    ///
    /// let parent = Model {
    ///     textures: Some(Textures::from(hashmap! {
    ///         "parent_texture" => "textures/parent",
    ///         "foo" => "#child_texture"
    ///     })),
    ///     ..Default::default()
    /// };
    ///
    /// // Provide models in increasing level of parenthood.
    /// let models = [child, parent];
    /// let resolved = ModelResolver::resolve_textures(models.iter());
    ///
    /// let expected = Textures::from(hashmap! {
    ///     "parent_texture" => "textures/parent",
    ///     "foo" => "textures/child",              // <------- resolved
    ///     "child_texture" => "textures/child",
    ///     "bar" => "textures/parent"              // <------- resolved    
    /// });
    ///
    /// assert_eq!(resolved, expected);
    /// ```
    pub fn resolve_textures<'a>(models: impl IntoIterator<Item = &'a Model>) -> Textures {
        let mut textures = Textures::default();

        for model in models.into_iter() {
            if let Some(mut parent_textures) = model.textures.clone() {
                // Resolve variables in the child using the parent textures first.
                textures.resolve(&parent_textures);

                // Then resolve variables in the parent using the child textures.
                parent_textures.resolve(&textures);

                // Merge them both.
                textures.merge(parent_textures.clone());
            }
        }

        textures
    }
}
