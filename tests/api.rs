#![cfg(feature = "tests")]

use minecraft_assets::api::AssetPack;

mod common;

use common::{model_path, single_variant_name, Flattening};

fn do_block_states_test(version: &str, flattening: Flattening) {
    let root =
        common::get_path_relative_to_manifest_dir(format!("tests/assets-{}", version)).unwrap();
    let assets = AssetPack::at_path(root);

    // Load the block states for `oak_planks`
    let states = assets.load_blockstates("oak_planks").unwrap();
    let variants = states.variants().unwrap();

    assert_eq!(variants.len(), 1);

    let model = &variants[&single_variant_name(flattening)].models()[0];
    assert_eq!(model.model, model_path("oak_planks", flattening));
}

#[test]
fn block_states_1_8() {
    do_block_states_test("1.8", Flattening::Pre);
}

#[test]
fn block_states_1_9() {
    do_block_states_test("1.9", Flattening::Pre);
}

#[test]
fn block_states_1_11() {
    do_block_states_test("1.11", Flattening::Pre);
}

#[test]
fn block_states_1_12() {
    do_block_states_test("1.12", Flattening::Pre);
}

#[test]
fn block_states_1_13() {
    do_block_states_test("1.13", Flattening::Post);
}

#[test]
fn block_states_1_14() {
    do_block_states_test("1.14", Flattening::Post);
}

#[test]
fn block_states_1_15() {
    do_block_states_test("1.15", Flattening::Post);
}
