#![cfg(feature = "tests")]

use maplit::hashmap;

use minecraft_assets::models::blockstates::{BlockStates, Model, Variant};

#[test]
fn blockstate_single_variant() {
    let b: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/oak_planks.json"
    ))
    .unwrap();

    assert_eq!(
        b,
        BlockStates {
            variants: hashmap! {
                String::from("") => Variant::Single(Model {
                    model: String::from("block/oak_planks"),
                    ..Default::default()
                })
            }
        }
    );
}

#[test]
fn blockstate_single_variant_multiple_models() {
    let b: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/stone.json"
    ))
    .unwrap();

    assert_eq!(
        b,
        BlockStates {
            variants: hashmap! {
                String::from("") => Variant::Multiple(vec![
                    Model {
                        model: String::from("block/stone"),
                        ..Default::default()
                    },
                    Model {
                        model: String::from("block/stone_mirrored"),
                        ..Default::default()
                    },
                    Model {
                        model: String::from("block/stone"),
                        y: Some(180),
                        ..Default::default()
                    },
                    Model {
                        model: String::from("block/stone_mirrored"),
                        y: Some(180),
                        ..Default::default()
                    }
                ])
            }
        }
    );
}

#[test]
fn blockstates_multiple_variants() {
    let b: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/stone_pressure_plate.json"
    ))
    .unwrap();

    assert_eq!(
        b,
        BlockStates {
            variants: hashmap! {
                String::from("powered=false") => Variant::Single(Model {
                    model: String::from("block/stone_pressure_plate"),
                    ..Default::default()
                }),

                String::from("powered=true") => Variant::Single(Model {
                    model: String::from("block/stone_pressure_plate_down"),
                    ..Default::default()
                })
            }
        }
    );
}
