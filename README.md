# minecraft-assets-rs

Parsing Minecraft asset files in Rust.

This crate provides types that can be used with [`serde_json`] to parse the
data files in the Minecraft `assets/` directory.

[`serde_json`]: https://lib.rs/crates/serde_json

## Documentation

Generate and view rustdocs by running:

```
$ cargo doc --open
```

## Tests

Integration tests in [`tests/`](tests/) use the actual asset files from the
[`minecraft-assets`] repository.

This repository is fairly large (~1 GB), so the tests in `tests/` do not run by
default. If you'd like to run them, use the [`tests/setup.sh`](tests/setup.sh)
script:

```
$ ./tests/setup.sh
```

This script will fetch the [`minecraft-assets`] repository and check out a few
different versions at various paths in [`tests/`](tests/). Then you can run the
tests by enabling the `tests` feature:

```
$ cargo test --features tests
```

[`minecraft-assets`]: https://github.com/InventivetalentDev

## License

This project is distributed under the terms of the MIT license.

Copyright Ben Reeves 2022
