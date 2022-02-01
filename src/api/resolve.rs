use crate::{
    api::{ModelIdentifier, ResourceIdentifier, ResourceLocation},
    schemas::models::{Model, Textures},
};

/// Methods for resolving the properties of a [`Model`] with respect to its
/// parents.
pub struct ModelResolver;

impl ModelResolver {
    /// Iterates through a [`Model`]'s parents to resolve all of the model's
    /// [texture variables].
    ///
    /// This works by merging together the [`Textures`] maps from all models in
    /// the parent-child chain, and then substituting texture variables with
    /// concrete values where possible.
    ///
    /// The `load_model` closure is passed the identifier of the next
    /// parent model to load and is expected to load or otherwise produce it.
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
    /// let models = hashmap! {
    ///     "parent" => Model {
    ///         textures: Some(Textures::from(hashmap! {
    ///             "parent_texture" => "textures/parent",
    ///             "foo" => "#child_texture"
    ///         })),
    ///         ..Default::default()
    ///     },
    ///     "child" => Model {
    ///         parent: Some(String::from("parent")),
    ///         textures: Some(Textures::from(hashmap! {
    ///             "child_texture" => "textures/child",
    ///             "bar" => "#parent_texture"
    ///         })),
    ///         ..Default::default()
    ///     }
    /// };
    ///
    /// let resolved = ModelResolver::resolve_textures(
    ///     &ResourceLocation::BlockModel("child".into()),
    ///     |next| -> Result<_, ()> {
    ///         let next = models[next.name()].clone();
    ///         Ok(next)
    ///     },
    /// ).unwrap();
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
    pub fn resolve_textures<F, E>(model: &ResourceLocation, load_model: F) -> Result<Textures, E>
    where
        F: FnMut(&ResourceLocation) -> Result<Model, E>,
    {
        let mut textures = Textures::default();

        Self::for_each_parent(
            model.clone(),
            |next| {
                if let Some(mut parent_textures) = next.textures {
                    println!("parent_textures: {:#?}", &parent_textures);
                    // Resolve variables in the child using the parent textures first.
                    textures.resolve(&parent_textures);

                    // Then resolve variables in the parent using the child textures.
                    parent_textures.resolve(&textures);

                    // Merge them both.
                    textures.merge(parent_textures);
                }
            },
            load_model,
        )?;

        Ok(textures)
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
                .map(|parent| ModelIdentifier::from(ResourceIdentifier::from(parent).into_owned()));

            op(model);

            match parent_owned {
                Some(parent) if !parent.is_builtin() => {
                    println!("{}", parent.as_str());
                    current = match current {
                        ResourceLocation::BlockModel(_) => ResourceLocation::BlockModel(parent),
                        ResourceLocation::ItemModel(_) => ResourceLocation::ItemModel(parent),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }

        Ok(())
    }
}
