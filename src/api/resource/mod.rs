mod identifier;
mod kind;
mod location;

pub use identifier::{ModelIdentifier, ResourceIdentifier, MINECRAFT_NAMESPACE};
pub use kind::{ResourceCategory, ResourceKind};
pub use location::ResourceLocation;
