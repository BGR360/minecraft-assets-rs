#![cfg(feature = "tests")]

use std::assert_eq;

use assert_matches::assert_matches;
use maplit::hashmap;

use minecraft_assets::models::blockstates::{
    multipart::{Case, Condition, WhenClause},
    BlockStates, Model, Variant,
};

macro_rules! condition {
    (
        $(
            $state:expr => $value:expr
        ),+
    ) => {
        Condition(hashmap! {
            $(
                $state.into() => $value.into()
            ),+
        })
    }
}

#[test]
fn blockstate_single_variant() {
    let actual: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/oak_planks.json"
    ))
    .unwrap();

    let expected = BlockStates::Variants {
        variants: hashmap! {
            String::from("") => Variant::Single(Model {
                model: String::from("block/oak_planks"),
                ..Default::default()
            })
        },
    };

    assert_eq!(actual, expected);
}

#[test]
fn blockstate_single_variant_multiple_models() {
    let actual: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/stone.json"
    ))
    .unwrap();

    let expected = BlockStates::Variants {
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
        },
    };

    assert_eq!(actual, expected);
}

#[test]
fn blockstates_multiple_variants() {
    let actual: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/stone_pressure_plate.json"
    ))
    .unwrap();

    let expected = BlockStates::Variants {
        variants: hashmap! {
            String::from("powered=false") => Variant::Single(Model {
                model: String::from("block/stone_pressure_plate"),
                ..Default::default()
            }),

            String::from("powered=true") => Variant::Single(Model {
                model: String::from("block/stone_pressure_plate_down"),
                ..Default::default()
            })
        },
    };

    assert_eq!(actual, expected);
}

#[test]
fn blockstates_multipart() {
    let actual: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/cobblestone_wall.json"
    ))
    .unwrap();

    let expected = BlockStates::Multipart {
        multipart: vec![
            Case {
                when: Some(WhenClause::Single(condition! { "up" => "true" })),
                apply: Variant::Single(Model {
                    model: String::from("block/cobblestone_wall_post"),
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "north" => "true" })),
                apply: Variant::Single(Model {
                    model: String::from("block/cobblestone_wall_side"),
                    uvlock: Some(true),
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "east" => "true" })),
                apply: Variant::Single(Model {
                    model: String::from("block/cobblestone_wall_side"),
                    uvlock: Some(true),
                    y: Some(90),
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "south" => "true" })),
                apply: Variant::Single(Model {
                    model: String::from("block/cobblestone_wall_side"),
                    uvlock: Some(true),
                    y: Some(180),
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "west" => "true" })),
                apply: Variant::Single(Model {
                    model: String::from("block/cobblestone_wall_side"),
                    uvlock: Some(true),
                    y: Some(270),
                    ..Default::default()
                }),
            },
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn blockstates_multipart_with_or() {
    let blockstates: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/redstone_wire.json"
    ))
    .unwrap();

    let expected_case = Case {
        when: Some(WhenClause::Or {
            or: vec![
                condition! {
                    "north" => "none",
                    "east" => "none",
                    "south" => "none",
                    "west" => "none"
                },
                condition! {"north" => "side|up", "east" => "side|up" },
                condition! {"east" => "side|up", "south" => "side|up" },
                condition! {"south" => "side|up", "west" => "side|up"},
                condition! {"west" => "side|up", "north" => "side|up"},
            ],
        }),
        apply: Variant::Single(Model {
            model: String::from("block/redstone_dust_dot"),
            ..Default::default()
        }),
    };

    assert_matches!(
        blockstates,
        BlockStates::Multipart { multipart } => {
            assert_eq!(multipart[0], expected_case);
        }
    );
}

// Some files don't have quotes around the boolean values in the "when" clauses.
// Make sure those can parse
#[test]
fn blockstates_multipart_with_boolean_values() {
    let blockstates: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14.4/assets/minecraft/blockstates/mossy_cobblestone_wall.json"
    ))
    .unwrap();

    let expected_case = Case {
        when: Some(WhenClause::Single(condition! { "up" => true })),
        apply: Variant::Single(Model {
            model: String::from("block/mossy_cobblestone_wall_post"),
            ..Default::default()
        }),
    };

    assert_matches!(
        blockstates,
        BlockStates::Multipart { multipart } => {
            assert_eq!(multipart[0], expected_case);
        }
    )
}
