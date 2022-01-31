#![cfg(feature = "tests")]

use minecraft_assets::api::{AssetPack, Result};

mod common;

#[test]
fn test_load_block_states() {
    let root = common::get_path_relative_to_manifest_dir("tests/assets-1.14").unwrap();
    let assets = AssetPack::at_path(root);

    // Load the block states for `oak_planks`
    let states = assets.load_blockstates("oak_planks").unwrap();
    let variants = states.variants().unwrap();

    assert_eq!(variants.len(), 1);

    let model = &variants[""].models()[0];
    assert_eq!(model.model, "block/oak_planks");
}
