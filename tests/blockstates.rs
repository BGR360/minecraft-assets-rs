#![cfg(feature = "tests")]

use assert_matches::assert_matches;
use maplit::hashmap;

use minecraft_assets::schemas::blockstates::{
    multipart::{Case, Condition, WhenClause},
    BlockStates, Model, Variant,
};

mod common;

macro_rules! condition {
    (
        $(
            $state:expr => $value:expr
        ),+
    ) => {
        Condition {
            and: hashmap! {
            $(
                $state.into() => $value.into()
            ),+
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum Version {
    Pre_1_13,
    Post_1_13,
}

// Prior to 1.13, single-variant blockstates had "normal" as their
// variant name. In versions >= 1.13, the variant name is ""
fn single_variant_name(version: Version) -> String {
    match version {
        Version::Pre_1_13 => String::from("normal"),
        Version::Post_1_13 => String::from(""),
    }
}

// In versions >= 1.13, model paths are prefixed with "block/".
fn model_path(model: &str, version: Version) -> String {
    match version {
        Version::Pre_1_13 => String::from(model),
        Version::Post_1_13 => format!("block/{}", model),
    }
}

fn do_single_variant_test(bytes: &[u8], version: Version) {
    let variant_name = single_variant_name(version);

    let expected = BlockStates::Variants {
        variants: hashmap! {
            variant_name => Variant::Single(Model {
                model: model_path("oak_planks", version),
                ..Default::default()
            })
        },
    };

    let actual: BlockStates = serde_json::from_slice(bytes).unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn single_variant_1_8() {
    do_single_variant_test(
        include_bytes!("./assets-1.8/assets/minecraft/blockstates/oak_planks.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_1_9() {
    do_single_variant_test(
        include_bytes!("./assets-1.9/assets/minecraft/blockstates/oak_planks.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_1_11() {
    do_single_variant_test(
        include_bytes!("./assets-1.11/assets/minecraft/blockstates/oak_planks.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_1_12() {
    do_single_variant_test(
        include_bytes!("./assets-1.12/assets/minecraft/blockstates/oak_planks.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_1_14() {
    do_single_variant_test(
        include_bytes!("./assets-1.14/assets/minecraft/blockstates/oak_planks.json"),
        Version::Post_1_13,
    );
}

fn do_single_variant_multiple_models_test(bytes: &[u8], version: Version) {
    let actual: BlockStates = serde_json::from_slice(bytes).unwrap();

    let expected = BlockStates::Variants {
        variants: hashmap! {
            single_variant_name(version) => Variant::Multiple(vec![
                Model {
                    model: model_path("stone", version),
                    ..Default::default()
                },
                Model {
                    model: model_path("stone_mirrored", version),
                    ..Default::default()
                },
                Model {
                    model: model_path("stone", version),
                    y: 180,
                    ..Default::default()
                },
                Model {
                    model: model_path("stone_mirrored", version),
                    y: 180,
                    ..Default::default()
                }
            ])
        },
    };

    assert_eq!(actual, expected);
}

#[test]
fn single_variant_multiple_models_1_8() {
    do_single_variant_multiple_models_test(
        include_bytes!("./assets-1.8/assets/minecraft/blockstates/stone.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_multiple_models_1_9() {
    do_single_variant_multiple_models_test(
        include_bytes!("./assets-1.9/assets/minecraft/blockstates/stone.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_multiple_models_1_11() {
    do_single_variant_multiple_models_test(
        include_bytes!("./assets-1.11/assets/minecraft/blockstates/stone.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_multiple_models_1_12() {
    do_single_variant_multiple_models_test(
        include_bytes!("./assets-1.12/assets/minecraft/blockstates/stone.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn single_variant_multiple_models_1_14() {
    do_single_variant_multiple_models_test(
        include_bytes!("./assets-1.14/assets/minecraft/blockstates/stone.json"),
        Version::Post_1_13,
    );
}

#[test]
fn multiple_variants() {
    let actual: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14/assets/minecraft/blockstates/stone_pressure_plate.json"
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

fn do_multipart_test(bytes: &[u8], version: Version) {
    let actual: BlockStates = serde_json::from_slice(bytes).unwrap();

    let expected = BlockStates::Multipart {
        multipart: vec![
            Case {
                when: Some(WhenClause::Single(condition! { "up" => "true" })),
                apply: Variant::Single(Model {
                    model: model_path("cobblestone_wall_post", version),
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "north" => "true" })),
                apply: Variant::Single(Model {
                    model: model_path("cobblestone_wall_side", version),
                    uv_lock: true,
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "east" => "true" })),
                apply: Variant::Single(Model {
                    model: model_path("cobblestone_wall_side", version),
                    uv_lock: true,
                    y: 90,
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "south" => "true" })),
                apply: Variant::Single(Model {
                    model: model_path("cobblestone_wall_side", version),
                    uv_lock: true,
                    y: 180,
                    ..Default::default()
                }),
            },
            Case {
                when: Some(WhenClause::Single(condition! { "west" => "true" })),
                apply: Variant::Single(Model {
                    model: model_path("cobblestone_wall_side", version),
                    uv_lock: true,
                    y: 270,
                    ..Default::default()
                }),
            },
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn multipart_1_9() {
    do_multipart_test(
        include_bytes!("./assets-1.9/assets/minecraft/blockstates/cobblestone_wall.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn multipart_1_11() {
    do_multipart_test(
        include_bytes!("./assets-1.11/assets/minecraft/blockstates/cobblestone_wall.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn multipart_1_12() {
    do_multipart_test(
        include_bytes!("./assets-1.12/assets/minecraft/blockstates/cobblestone_wall.json"),
        Version::Pre_1_13,
    );
}

#[test]
fn multipart_1_14() {
    do_multipart_test(
        include_bytes!("./assets-1.14/assets/minecraft/blockstates/cobblestone_wall.json"),
        Version::Post_1_13,
    );
}

#[test]
fn multipart_with_or() {
    let blockstates: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14/assets/minecraft/blockstates/redstone_wire.json"
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
fn multipart_with_boolean_values() {
    let blockstates: BlockStates = serde_json::from_slice(include_bytes!(
        "./assets-1.14/assets/minecraft/blockstates/mossy_cobblestone_wall.json"
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

fn parse_all_blockstates_in_version(version: &str) {
    common::parse_all_in_dir::<BlockStates>(&format!(
        "tests/assets-{}/assets/minecraft/blockstates",
        version
    ));
}

#[test]
fn can_parse_all_blockstates_1_8() {
    parse_all_blockstates_in_version("1.8");
}

#[test]
fn can_parse_all_blockstates_1_9() {
    parse_all_blockstates_in_version("1.9");
}

#[test]
fn can_parse_all_blockstates_1_11() {
    parse_all_blockstates_in_version("1.11");
}

#[test]
fn can_parse_all_blockstates_1_12() {
    parse_all_blockstates_in_version("1.12");
}

#[test]
fn can_parse_all_blockstates_1_14() {
    parse_all_blockstates_in_version("1.14");
}
