# gitignore-template-generator

[<img alt="github" src="https://img.shields.io/badge/github-black?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Patacode/gitignore-template-generator)
[<img alt="crates.io" src="https://img.shields.io/crates/v/gitignore-template-generator?logoColor=E3A835&style=for-the-badge&color=9c7325&logo=rust" height="20">](https://crates.io/crates/gitignore-template-generator)
[<img alt="crates.io" src="https://img.shields.io/crates/d/gitignore-template-generator?logoColor=E3A835&style=for-the-badge&color=152673" height="20">](https://crates.io/crates/gitignore-template-generator)

A binary crate to generate templates for .gitignore files

## Installation

Current release: [0.2.0](CHANGELOG.md#0.2.0)

```bash
cargo install gitignore-template-generator
```

## Usage

Available options:

```
$ gitignore-template-generator --help
Usage: gitignore-template-generator [OPTIONS] <TEMPLATE_NAMES>...

Generate templates for .gitignore files

Arguments:
  <TEMPLATE_NAMES>...  A non-empty list of existing gitignore template names

Options:
  -a, --author   Print author
  -h, --help     Print help
  -V, --version  Print version

Version: 0.2.0
Author: Patacode <pata.codegineer@gmail.com>
```

Examples:

*Generate a gitignore template for simple rust projects:*

```
$ gitignore-template-generator rust
# Created by https://www.toptal.com/developers/gitignore/api/rust
# Edit at https://www.toptal.com/developers/gitignore?templates=rust

### Rust ###
# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb

# End of https://www.toptal.com/developers/gitignore/api/rust
```

## Development

Install `cargo-make`, dev tools and build the package:

```bash
./bootstrap.sh
```

Run the tests:

```bash
cargo test # unit + integration tests
cargo test --lib # unit tests
cargo test --test integration_tests # integration tests
```

Generate code coverage report in HTML format under `target/tarpaulin`:

```bash
cargo tarpaulin \
  --out Html \
  --exclude-files src/lib.rs src/main.rs benches/* tests/* \
  --output-dir target/tarpaulin
```

Benchmark binary and library crates:

```bash
cargo bench
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
