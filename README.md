# minecraft-assets

[![Crates.io](https://img.shields.io/crates/v/minecraft-assets.svg)](https://crates.io/crates/minecraft-assets)
[![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/minecraft-assets)

A library for reading Minecraft asset files and resource packs in Rust.

## Example

```rust,no_run
use minecraft_assets::api::AssetPack;

let assets = AssetPack::at_path("~/.minecraft/");

// Load the block states for `oak_planks`
let states = assets.load_blockstates("oak_planks").unwrap();
let variants = states.variants().unwrap();

assert_eq!(variants.len(), 1);

let model_properties = &variants[""].models()[0];
assert_eq!(model_properties.model, "block/oak_planks");
```

## Documentation

* [Main Branch (github.io)](https://bgr360.github.io/minecraft-assets-rs/minecraft_assets/)
* [Latest Release (docs.rs)](https://docs.rs/minecraft-assets)

## Tests

Integration tests in [`tests/`](tests/) use the actual asset files from the
[`minecraft-assets`] repository.

That repository is fairly large (~1 GB), so the tests in `tests/` do not run by
default. If you'd like to run them, use the [`tests/setup.sh`](tests/setup.sh)
script:

```txt
$ ./tests/setup.sh
```

This script will fetch the [`minecraft-assets`] repository and check out a few
different versions at various paths in [`tests/`](tests/). Then you can run the
tests by enabling the `tests` feature:

```txt
$ cargo test --features tests
```

[`minecraft-assets`]: https://github.com/InventivetalentDev

## License

This project is distributed under the terms of the MIT license.

Copyright Ben Reeves 2022
