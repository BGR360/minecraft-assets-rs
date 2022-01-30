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

        println!("Parsing {}", path.to_string_lossy());

        let file = fs::File::open(path).unwrap();
        serde_json::from_reader::<_, T>(file).unwrap();
    }
}
