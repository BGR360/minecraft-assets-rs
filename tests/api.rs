#![cfg(feature = "tests")]

use minecraft_assets::api::{AssetPack, ModelIdentifier};

mod common;

use common::{single_variant_name, Flattening};

fn load_block_states(assets: &AssetPack, flattening: Flattening) {
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
}

fn do_api_test(version: &str, flattening: Flattening) {
    let root =
        common::get_path_relative_to_manifest_dir(format!("tests/assets-{}", version)).unwrap();
    let assets = AssetPack::at_path(root);

    load_block_states(&assets, flattening);
    load_block_model(&assets);
}

#[test]
fn api_1_8() {
    do_api_test("1.8", Flattening::Pre);
}

#[test]
fn api_1_9() {
    do_api_test("1.9", Flattening::Pre);
}

#[test]
fn api_1_11() {
    do_api_test("1.11", Flattening::Pre);
}

#[test]
fn api_1_12() {
    do_api_test("1.12", Flattening::Pre);
}

#[test]
fn api_1_13() {
    do_api_test("1.13", Flattening::Post);
}

#[test]
fn api_1_14() {
    do_api_test("1.14", Flattening::Post);
}

#[test]
fn api_1_15() {
    do_api_test("1.15", Flattening::Post);
}
