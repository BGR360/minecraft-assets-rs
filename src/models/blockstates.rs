//! Serde-(de)serializable data types for `blockstates/*.json`.
//!
//! See <https://minecraft.fandom.com/wiki/Model#Block_states>.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Block states as stored in `blockstates/{}.json`.
///
/// There are several different variants of some blocks (like [doors], which can
/// be open or closed), hence each block has its own [block state] file, which
/// lists all its existing variants and links them to their corresponding
/// models. Blocks can also be compound of several different models at the same
/// time, called "multipart". The models are then used depending on the block
/// states of the block.
///
/// These files are stored in the following folder:
/// `assets/<namespace>/blockstates`. The files are used directly based on their
/// filename, thus a block state file with another name than the existing ones
/// does not affect any block.
///
/// See also the corresponding section of the [wiki page].
///
/// [doors]: https://minecraft.fandom.com/wiki/Door
/// [block state]: https://minecraft.fandom.com/wiki/Block_state
/// [wiki page]: <https://minecraft.fandom.com/wiki/Model#Block_states>
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct BlockStates {
    /// Holds all the variants of the block by name.
    ///
    /// The variant name consists of the relevant block states separated by
    /// commas, for example, `"face=wall,facing=east,powered=false"`.
    ///
    /// A block with just one variant uses `""` as the name for its variant.
    pub variants: HashMap<String, Variant>,
}

/// A block variant.
///
/// Each variant can have one model or an array of models and contains their
/// properties. If set to an array, the model is chosen randomly from the models
/// contained in the array based on the `Model::weight` field.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Variant {
    /// A variant with only a single model to choose from.
    Single(Model),

    /// A variant with multiple models to choose from.
    Multiple(Vec<Model>),
}

impl Default for Variant {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

impl Variant {
    /// Returns all of the possible [`Model`]s for this variant as a slice.
    ///
    /// The slice will contain one element for a [`Single`][Self::Single]
    /// variant, and multiple for a [`Multiple`][Self::Multiple] variant.
    pub fn models(&self) -> &[Model] {
        match self {
            Self::Single(model) => std::slice::from_ref(model),
            Self::Multiple(models) => &models[..],
        }
    }
}

/// Contains the properties of a model.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Model {
    /// Specifies the path to the model file of the block, in form of a
    /// [resource location].
    ///
    /// [resource location]: <https://minecraft.fandom.com/wiki/Model#File_path>
    pub model: String,

    /// Rotation of the model on the x-axis in increments of 90 degrees.
    pub x: Option<i32>,

    /// Rotation of the model on the y-axis in increments of 90 degrees.
    pub y: Option<i32>,

    /// Can be `true` or `false` (default). Locks the rotation of the texture of
    /// a block, if set to `true`. This way the texture does not rotate with the
    /// block when using the [`x`] and [`y`] fields above.
    ///
    /// See the example on the [wiki page].
    ///
    /// [wiki page]: <https://minecraft.fandom.com/wiki/Model#Block_states>
    pub uvlock: Option<bool>,

    /// Sets the probability of the model for being used in the game.
    ///
    /// The weight defaults to 1 (=100%). If more than one model is used for the
    /// same variant, the probability is calculated by dividing the individual
    /// model's weight by the sum of the weights of all models. (For example, if
    /// three models are used with weights 1, 1, and 2, then their combined
    /// weight would be 4 (1+1+2). The probability of each model being used
    /// would then be determined by dividing each weight by 4: 1/4, 1/4 and 2/4,
    /// or 25%, 25% and 50%, respectively.)
    pub weight: Option<u32>,
}
