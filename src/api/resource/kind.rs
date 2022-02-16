#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCategory {
    /// Resources located in the `assets/` directory.
    Assets,

    /// Resource located in the `data/` directory.
    Data,
}

impl ResourceCategory {
    /// Returns the name of the top-level directory containing this category of
    /// resource.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let category = ResourceCategory::Assets;
    /// assert_eq!(category.directory(), "assets");
    ///
    /// let category = ResourceCategory::Data;
    /// assert_eq!(category.directory(), "data");
    /// ```
    pub fn directory(&self) -> &'static str {
        match self {
            Self::Assets => "assets",
            Self::Data => "data",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Resources (`.json`) in `assets/<namespace>/blockstates/`.
    BlockStates,

    /// Resources (`.json`) in `assets/<namespace>/models/block/`.
    BlockModel,

    /// Resources (`.json`) in `assets/<namespace>/models/item/`.
    ItemModel,

    /// Resources (`.png`) in `assets/<namespace>/textures/`.
    Texture,

    /// Resources (`.mcmeta`) in `assets/<namespace>/textures/`.
    TextureMeta,
}

impl ResourceKind {
    /// Returns the category of this resource type (assets or data).
    pub fn category(&self) -> ResourceCategory {
        match self {
            Self::BlockStates
            | Self::BlockModel
            | Self::ItemModel
            | Self::Texture
            | Self::TextureMeta => ResourceCategory::Assets,
        }
    }

    /// Returns the file extension used for this resource's file.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let kind = ResourceKind::BlockStates;
    /// assert_eq!(kind.extension(), "json");
    ///
    /// let kind = ResourceKind::Texture;
    /// assert_eq!(kind.extension(), "png");
    ///
    /// let kind = ResourceKind::TextureMeta;
    /// assert_eq!(kind.extension(), "mcmeta");
    /// ```
    pub fn extension(&self) -> &'static str {
        match self {
            Self::BlockStates | Self::BlockModel | Self::ItemModel => "json",
            Self::Texture => "png",
            Self::TextureMeta => "mcmeta",
        }
    }

    /// Returns the path relative to `assets/<namespace>/` or
    /// `data/<namespace>/` in which resources of this type reside.
    pub fn directory(&self) -> &'static str {
        match self {
            Self::BlockStates => "blockstates",
            Self::BlockModel => "models/block",
            Self::ItemModel => "models/item",
            Self::Texture | Self::TextureMeta => "textures",
        }
    }
}
