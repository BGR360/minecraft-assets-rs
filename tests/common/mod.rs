use std::{
    env, fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

pub fn get_path_relative_to_manifest_dir(
    relative_path: impl AsRef<Path>,
) -> Result<PathBuf, env::VarError> {
    let manifest_path = env::var("CARGO_MANIFEST_DIR")?;

    let mut path = PathBuf::from(manifest_path);
    path.push(relative_path);

    Ok(path)
}

pub fn parse_all_in_dir<T: for<'de> Deserialize<'de>>(path: &str) {
    let dir_path = get_path_relative_to_manifest_dir(path).unwrap();

    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.file_name().unwrap().to_string_lossy().starts_with('_') {
            continue;
        }

        println!("Parsing {}", path.to_string_lossy());

        let file = fs::File::open(path).unwrap();
        serde_json::from_reader::<_, T>(file).unwrap();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Versions {
    PreFlattening,
    PostFlattening,
    Post_1_16_2,
}

// Prior to 1.13, single-variant blockstates had "normal" as their
// variant name. In versions >= 1.13, the variant name is ""
pub fn single_variant_name(version: Versions) -> String {
    match version {
        Versions::PreFlattening => String::from("normal"),
        Versions::PostFlattening | Versions::Post_1_16_2 => String::from(""),
    }
}

// In versions >= 1.13, model paths are prefixed with "block/".
pub fn model_path(model: &str, version: Versions) -> String {
    match version {
        Versions::PreFlattening => String::from(model),
        Versions::PostFlattening => format!("block/{}", model),
        Versions::Post_1_16_2 => format!("minecraft:block/{}", model),
    }
}
