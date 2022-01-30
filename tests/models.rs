#![cfg(feature = "tests")]

use std::assert_eq;

use maplit::hashmap;

use minecraft_assets::schemas::models::{Model, Textures};

fn do_cube_all_test(bytes: &[u8]) {
    let actual: Model = serde_json::from_slice(bytes).unwrap();

    let expected = Model {
        parent: Some(String::from("block/cube")),
        textures: Some(Textures {
            variables: hashmap! {
                String::from("particle") => "#all".into(),
                String::from("down") => "#all".into(),
                String::from("up") => "#all".into(),
                String::from("north") => "#all".into(),
                String::from("east") => "#all".into(),
                String::from("south") => "#all".into(),
                String::from("west") => "#all".into(),
            },
        }),
        ..Default::default()
    };

    assert_eq!(actual, expected);
}

#[test]
fn cube_all_1_12_2() {
    do_cube_all_test(include_bytes!(
        "./assets-1.12.2/assets/minecraft/models/block/cube_all.json"
    ));
}

#[test]
fn cube_all_1_14_4() {
    do_cube_all_test(include_bytes!(
        "./assets-1.14.4/assets/minecraft/models/block/cube_all.json"
    ));
}
