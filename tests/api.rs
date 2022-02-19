#![cfg(feature = "tests")]

use assert_matches::assert_matches;
use std::{io, path::PathBuf};

use minecraft_assets::api::{
    AssetPack, EnumerateResources, FileSystemResourceProvider, ModelIdentifier, ResourceKind,
    ResourceProvider,
};

mod common;

use common::{single_variant_name, Versions};

fn get_assets_root(version: &str) -> PathBuf {
    common::get_path_relative_to_manifest_dir(format!("tests/assets-{}", version)).unwrap()
}

#[test]
fn test_resource_iter() {
    let provider = FileSystemResourceProvider::new(get_assets_root("1.14"));

    let textures = provider
        .enumerate_resources("minecraft", ResourceKind::Texture)
        .unwrap();

    for needle in [
        "block/kelp",
        "item/diamond_hoe",
        "entity/alex",
        "entity/llama/decor/blue",
        "gui/advancements/backgrounds/adventure",
    ] {
        assert!(textures.iter().find(|id| id.as_str() == needle).is_some());
    }

    let texture_metas = provider
        .enumerate_resources("minecraft", ResourceKind::TextureMeta)
        .unwrap();

    for needle in ["block/kelp", "block/campfire_fire", "block/lava_flow"] {
        assert!(texture_metas
            .iter()
            .find(|id| id.as_str() == needle)
            .is_some());
    }
}

fn load_block_states(assets: &AssetPack, flattening: Versions) {
    let states = assets.load_blockstates("oak_planks").unwrap();
    let variants = states.variants().unwrap();

    assert_eq!(variants.len(), 1);

    let model = &variants[&single_variant_name(flattening)].models()[0];
    assert_eq!(ModelIdentifier::model_name(&model.model), "oak_planks");
}

fn load_block_model(assets: &AssetPack) {
    // Try it with both a prefixed and non-prefixed path (both should work on
    // all versions).
    let model = assets.load_block_model("cube_all").unwrap();
    assert_eq!(ModelIdentifier::model_name(&model.parent.unwrap()), "cube");

    let model = assets.load_block_model("block/cube_all").unwrap();
    assert_eq!(ModelIdentifier::model_name(&model.parent.unwrap()), "cube");

    // Item model should fail.
    assert_matches!(assets.load_block_model("diamond_hoe"), Err(_));
}

fn load_item_model(assets: &AssetPack, version: &str) {
    let expected_parent = if version == "1.8" {
        "generated"
    } else {
        "handheld"
    };

    // Try it with both a prefixed and non-prefixed path (both should work on
    // all versions).
    let model = assets.load_item_model("diamond_hoe").unwrap();
    assert_eq!(
        ModelIdentifier::model_name(&model.parent.unwrap()),
        expected_parent
    );

    let model = assets.load_item_model("item/diamond_hoe").unwrap();
    assert_eq!(
        ModelIdentifier::model_name(&model.parent.unwrap()),
        expected_parent
    );

    // Block model should fail.
    assert_matches!(assets.load_item_model("cube_all"), Err(_));
}

fn load_block_model_recursive(assets: &AssetPack, version: &str) {
    let models = assets.load_block_model_recursive("cube_all").unwrap();

    let expected = if version == "1.8" {
        vec![
            assets.load_block_model("cube_all").unwrap(),
            assets.load_block_model("cube").unwrap(),
        ]
    } else {
        vec![
            assets.load_block_model("cube_all").unwrap(),
            assets.load_block_model("cube").unwrap(),
            assets.load_block_model("block").unwrap(),
        ]
    };

    assert_eq!(models, expected);
}

fn load_item_model_recursive(assets: &AssetPack, version: &str) {
    let models = assets.load_item_model_recursive("diamond_hoe").unwrap();

    let expected = if version == "1.8" {
        vec![
            assets.load_item_model("item/diamond_hoe").unwrap(),
            //assets.load_item_model("builtin/generated").unwrap(),
        ]
    } else {
        vec![
            assets.load_item_model("item/diamond_hoe").unwrap(),
            assets.load_item_model("item/handheld").unwrap(),
            assets.load_item_model("item/generated").unwrap(),
            // assets.load_item_model("builtin/generated").unwrap(),
        ]
    };

    assert_eq!(models, expected);
}

fn do_api_test(version: &str, flattening: Versions) {
    let assets = get_asset_pack(version);

    load_block_states(&assets, flattening);
    load_block_model(&assets);
    load_item_model(&assets, version);
    load_block_model_recursive(&assets, version);
    load_item_model_recursive(&assets, version);
}

fn get_asset_pack(version: &str) -> AssetPack {
    let root = get_assets_root(version);
    AssetPack::at_path(root)
}

#[test]
fn api_1_8() {
    do_api_test("1.8", Versions::PreFlattening);
}

#[test]
fn api_1_9() {
    do_api_test("1.9", Versions::PreFlattening);
}

#[test]
fn api_1_11() {
    do_api_test("1.11", Versions::PreFlattening);
}

#[test]
fn api_1_12() {
    do_api_test("1.12", Versions::PreFlattening);
}

#[test]
fn api_1_13() {
    do_api_test("1.13", Versions::PostFlattening);
}

#[test]
fn api_1_14() {
    do_api_test("1.14", Versions::PostFlattening);
}

#[test]
fn api_1_15() {
    do_api_test("1.15", Versions::PostFlattening);
}

#[test]
fn api_1_16_2() {
    do_api_test("1.16.2", Versions::Post_1_16_2);
}

#[test]
fn api_1_17() {
    do_api_test("1.17", Versions::Post_1_16_2);
}

#[test]
fn api_1_18() {
    do_api_test("1.18", Versions::Post_1_16_2);
}

#[test]
fn for_each_blockstates() {
    let assets = get_asset_pack("1.14");

    assert_eq!(
        assets
            .enumerate_resources("minecraft", ResourceKind::BlockStates)
            .unwrap()
            .len(),
        677
    );
}

#[test]
fn for_each_block_model() {
    let assets = get_asset_pack("1.14");

    assert_eq!(
        assets
            .enumerate_resources("minecraft", ResourceKind::BlockModel)
            .unwrap()
            .len(),
        1201
    );
}

#[test]
fn for_each_item_model() {
    let assets = get_asset_pack("1.14");

    assert_eq!(
        assets
            .enumerate_resources("minecraft", ResourceKind::ItemModel)
            .unwrap()
            .len(),
        1006,
    );
}

#[test]
fn for_each_texture() {
    let assets = get_asset_pack("1.14");

    assert_eq!(
        assets
            .enumerate_resources("minecraft", ResourceKind::Texture)
            .unwrap()
            .len(),
        1889,
    );
}

#[test]
fn for_each_texture_meta() {
    let assets = get_asset_pack("1.14");

    assert_eq!(
        assets
            .enumerate_resources("minecraft", ResourceKind::TextureMeta)
            .unwrap()
            .len(),
        54,
    );
}
