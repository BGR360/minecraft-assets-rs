//! Serde-(de)serializable data types for `blockstates/*.json`.
//!
//! See <https://minecraft.fandom.com/wiki/Model#Block_states>.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Block states as stored in the `assets/<namespace>/blockstates` directory.
///
/// There are several different variants of some blocks (like [doors], which can
/// be open or closed), hence each block has its own [block state] file, which
/// lists all its existing variants and links them to their corresponding
/// models.
///
/// Blocks can also be compound of several different models at the same
/// time, called "multipart". The models are then used depending on the block
/// states of the block.
///
/// See also the corresponding section of the [wiki page].
///
/// [doors]: https://minecraft.fandom.com/wiki/Door
/// [block state]: https://minecraft.fandom.com/wiki/Block_state
/// [wiki page]: <https://minecraft.fandom.com/wiki/Model#Block_states>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum BlockStates {
    /// One way of representing the different states of a block.
    ///
    /// This uses a map from variant name to block variant. The variant name
    /// consists of the relevant block states separated by commas, for example,
    /// `"face=wall,facing=east,powered=false"`.
    ///
    /// A block with just one variant uses `""` as the name for its variant.
    Variants {
        /// Holds all the variants of the block by name.
        variants: HashMap<String, Variant>,
    },

    /// Another way of representing the different states of a block.
    ///
    /// This uses a list of "cases" that specify when a particular model should
    /// apply.
    Multipart {
        /// Holds all the cases and the models that should apply in each case.
        multipart: Vec<multipart::Case>,
    },
}

impl BlockStates {
    /// Returns the mapping from block states to [`Variant`]s, or `None` if the
    /// block states are specified as [`Multipart`].
    ///
    /// [`Multipart`]: Self::Multipart
    pub fn variants(&self) -> Option<&HashMap<String, Variant>> {
        match self {
            Self::Variants { ref variants } => Some(variants),
            Self::Multipart { .. } => None,
        }
    }

    /// Returns the list of [`Case`]s that specify how to display the different
    /// [`Variant`]s, or `None` if the block states are specified as
    /// [`Variants`].
    ///
    /// [`Case`]: multipart::Case
    /// [`Variants`]: Self::Variants
    pub fn cases(&self) -> Option<&[multipart::Case]> {
        match self {
            Self::Variants { .. } => None,
            Self::Multipart { multipart } => Some(&multipart[..]),
        }
    }
}

impl Default for BlockStates {
    fn default() -> Self {
        Self::Variants {
            variants: Default::default(),
        }
    }
}

/// A block variant.
///
/// Each variant can have **one model** or an **array of models** and contains
/// their properties. If set to an array, the model is chosen randomly from the
/// models contained in the array based on the `Model::weight` field.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Variant {
    /// A variant with only a single model to choose from.
    Single(ModelProperties),

    /// A variant with multiple models to choose from.
    Multiple(Vec<ModelProperties>),
}

impl Default for Variant {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

impl Variant {
    /// Returns all of the possible [`ModelProperties`] choices for this variant
    /// as a slice.
    ///
    /// The slice will contain one element for a [`Single`][Self::Single]
    /// variant, and multiple for a [`Multiple`][Self::Multiple] variant.
    pub fn models(&self) -> &[ModelProperties] {
        match self {
            Self::Single(model) => std::slice::from_ref(model),
            Self::Multiple(models) => &models[..],
        }
    }
}

/// Contains the properties of a model that is used to render all or part of a
/// block in a particular state.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelProperties {
    /// Specifies the path to the model file of the block, in the form of a
    /// [resource location].
    ///
    /// # Version note
    ///
    /// In version 1.13 and beyond, this path includes a prefix such as `block/`
    /// or `item/` to disambiguate between different model types. Versions prior
    /// to 1.13 do not include this.
    ///
    /// See the [`ModelIdentifier`] documentation for more information.
    ///
    /// [resource location]: <https://minecraft.fandom.com/wiki/Model#File_path>
    /// [`ModelIdentifier`]: crate::api::ModelIdentifier
    pub model: String,

    /// Rotation of the model on the x-axis in increments of 90 degrees.
    #[serde(default = "ModelProperties::default_rotation")]
    pub x: i32,

    /// Rotation of the model on the y-axis in increments of 90 degrees.
    #[serde(default = "ModelProperties::default_rotation")]
    pub y: i32,

    /// Can be `true` or `false` (default). Locks the rotation of the texture of
    /// a block, if set to `true`. This way the texture does not rotate with the
    /// block when using the `x` and `y` fields above.
    ///
    /// See the example on the [wiki page].
    ///
    /// [wiki page]: <https://minecraft.fandom.com/wiki/Model#Block_states>
    #[serde(rename = "uvlock", default = "ModelProperties::default_uv_lock")]
    pub uv_lock: bool,

    /// Sets the probability of the model for being used in the game.
    ///
    /// The weight defaults to 1 (=100%). If more than one model is used for the
    /// same variant, the probability is calculated by dividing the individual
    /// model's weight by the sum of the weights of all models. (For example, if
    /// three models are used with weights 1, 1, and 2, then their combined
    /// weight would be 4 (1+1+2). The probability of each model being used
    /// would then be determined by dividing each weight by 4: 1/4, 1/4 and 2/4,
    /// or 25%, 25% and 50%, respectively.)
    #[serde(default = "ModelProperties::default_weight")]
    pub weight: u32,
}

impl ModelProperties {
    pub(crate) const fn default_rotation() -> i32 {
        0
    }

    pub(crate) const fn default_uv_lock() -> bool {
        false
    }

    pub(crate) const fn default_weight() -> u32 {
        1
    }
}

impl Default for ModelProperties {
    fn default() -> Self {
        Self {
            model: Default::default(),
            x: Self::default_rotation(),
            y: Self::default_rotation(),
            uv_lock: Self::default_uv_lock(),
            weight: Self::default_weight(),
        }
    }
}

/// Types used to compose [`BlockStates::Multipart`].
pub mod multipart {
    use super::*;

    /// Specifies a case and the model that should apply in that case.
    #[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
    pub struct Case {
        /// A list of cases that have to be met for the model to be applied.
        ///
        /// If unset, the model always applies.
        pub when: Option<WhenClause>,

        /// Specifies the model(s) to apply and its properties.
        pub apply: Variant,
    }

    /// A list of conditions that have to be met for a model to be applied.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    #[serde(untagged)]
    pub enum WhenClause {
        /// A `when` clause that is true when the given condition is true.
        Single(Condition),

        /// A `when` clause that is true when any of the given conditions is true.
        Or {
            /// The conditions in the `OR` clause.
            #[serde(rename = "OR")]
            or: Vec<Condition>,
        },
    }

    impl WhenClause {
        /// Returns all of the [`Condition`]s of this when clause as a slice.
        ///
        /// The slice will contain one element for a [`Single`][Self::Single]
        /// variant, and multiple for an [`Or`][Self::Or] variant.
        pub fn conditions(&self) -> &[Condition] {
            match self {
                Self::Single(condition) => std::slice::from_ref(condition),
                Self::Or { or } => &or[..],
            }
        }
    }

    /// A set of conditions that **all** have to match the block to return true.
    ///
    /// The condition is structured as a map from `state` to `value`, so for instance:
    ///
    /// ```json
    /// "when": {"north": "side|up", "east": "side|up" }
    /// ```
    #[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
    pub struct Condition {
        /// Map from state name to state value that forms the list of conditions.
        #[serde(flatten)]
        pub and: HashMap<String, StateValue>,
    }

    /// The right-hand side of a [`Condition`] requirement.
    ///
    /// ```txt
    /// "when": {"north": "side|up", "east": false }
    ///                   ^^^^^^^^^          ^^^^^
    /// ```
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    #[serde(untagged)]
    pub enum StateValue {
        /// Unquoted bool value.
        Bool(bool),

        /// String value (possibly boolean-like, i.e., `"true"` or `"false"`).
        String(String),
    }

    impl From<bool> for StateValue {
        fn from(source: bool) -> Self {
            Self::Bool(source)
        }
    }

    impl<'a> From<&'a str> for StateValue {
        fn from(source: &'a str) -> Self {
            Self::String(String::from(source))
        }
    }

    impl From<String> for StateValue {
        fn from(source: String) -> Self {
            Self::String(source)
        }
    }
}
