#![cfg(feature = "tests")]

use assert_matches::assert_matches;
use std::{io, path::PathBuf};

use minecraft_assets::api::{AssetPack, ModelIdentifier};

mod common;

use common::{single_variant_name, Versions};

fn load_block_states(assets: &AssetPack, flattening: Versions) {
    let states = assets.load_blockstates("oak_planks").unwrap();
    let variants = states.variants().unwrap();

    assert_eq!(variants.len(), 1);

    let model = &variants[&single_variant_name(flattening)].models()[0];
    assert_eq!(
        ModelIdentifier::from(&model.model),
        ModelIdentifier::from("oak_planks")
    );
}

fn load_block_model(assets: &AssetPack) {
    // Try it with both a prefixed and non-prefixed path (both should work on
    // all versions).
    let model = assets.load_block_model("cube_all").unwrap();
    assert_eq!(
        ModelIdentifier::from(&model.parent.unwrap()),
        ModelIdentifier::from("cube")
    );

    let model = assets.load_block_model("block/cube_all").unwrap();
    assert_eq!(
        ModelIdentifier::from(&model.parent.unwrap()),
        ModelIdentifier::from("cube")
    );

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
        ModelIdentifier::from(&model.parent.unwrap()),
        ModelIdentifier::from(expected_parent)
    );

    let model = assets.load_item_model("item/diamond_hoe").unwrap();
    assert_eq!(
        ModelIdentifier::from(&model.parent.unwrap()),
        ModelIdentifier::from(expected_parent)
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
    let root =
        common::get_path_relative_to_manifest_dir(format!("tests/assets-{}", version)).unwrap();
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
    do_api_test("1.16.2", Versions::PostFlattening);
}

#[test]
fn api_1_17() {
    do_api_test("1.17", Versions::PostFlattening);
}

#[test]
fn api_1_18() {
    do_api_test("1.18", Versions::PostFlattening);
}

#[test]
fn for_each_blockstates() {
    let assets = get_asset_pack("1.14");

    let mut paths = Vec::new();
    assets
        .for_each_blockstates(|_, path| -> Result<(), io::Error> {
            paths.push(PathBuf::from(path));
            Ok(())
        })
        .unwrap();

    assert_eq!(paths.len(), 677);
}

#[test]
fn for_each_block_model() {
    let assets = get_asset_pack("1.14");

    let mut paths = Vec::new();
    assets
        .for_each_block_model(|_, path| -> Result<(), io::Error> {
            paths.push(PathBuf::from(path));
            Ok(())
        })
        .unwrap();

    assert_eq!(paths.len(), 1201);
}

#[test]
fn for_each_item_model() {
    let assets = get_asset_pack("1.14");

    let mut paths = Vec::new();
    assets
        .for_each_item_model(|_, path| -> Result<(), io::Error> {
            paths.push(PathBuf::from(path));
            Ok(())
        })
        .unwrap();

    assert_eq!(paths.len(), 1006);
}

#[test]
fn for_each_blockstates_with_error() {
    let assets = get_asset_pack("1.14");

    assert_matches!(
        assets.for_each_item_model(|_, _| Err(io::Error::new(io::ErrorKind::Other, ""))),
        Err(_)
    );
}
