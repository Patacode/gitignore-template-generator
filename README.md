# gitignore-template-generator

[<img alt="github" src="https://img.shields.io/badge/github-black?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Patacode/gitignore-template-generator)
[<img alt="crates.io" src="https://img.shields.io/crates/v/gitignore-template-generator?logoColor=E3A835&style=for-the-badge&color=9c7325&logo=rust" height="20">](https://crates.io/crates/gitignore-template-generator)
[<img alt="crates.io" src="https://img.shields.io/crates/d/gitignore-template-generator?logoColor=E3A835&style=for-the-badge&color=152673" height="20">](https://crates.io/crates/gitignore-template-generator)

A binary crate to generate templates for .gitignore files

## Installation

Current release: [0.8.0](CHANGELOG.md#0.8.0)

```bash
cargo install gitignore-template-generator
```

## Usage

Available options:

```
Usage: gitignore-template-generator [OPTIONS] [TEMPLATE_NAMES]...

Generate templates for .gitignore files

Arguments:
  [TEMPLATE_NAMES]...  A non-empty list of gitignore template names

Options:
  -s, --server-url <SERVER_URL>        The template manager url [default: https://www.toptal.com]
  -g, --generator-uri <GENERATOR_URI>  The template generator uri [default: /developers/gitignore/api]
  -i, --lister-uri <LISTER_URI>        The template lister uri [default: /developers/gitignore/api/list]
  -l, --list                           List available templates
  -c, --check                          Enable robust template names check
  -h, --help                           Print help
  -V, --version                        Print version
  -a, --author                         Print author

Version: 0.8.0
Author: Patacode <pata.codegineer@gmail.com>
```

The cli tool binds to any template manager service able to manage templates
over HTTP. It defaults to `toptal` template manager, using
`https://www.toptal.com/developers/gitignore/api/{templateNames}` to generate
templates (with `{templateNames}` being a comma-separated list of template
names), and `https://www.toptal.com/developers/gitignore/api/list` to list
them.

It is possible to provide a custom template manager service to the cli
tool. One can easily change the template manager service base url and related
endpoint uris using the `server-url`, `generator-uri` and `lister-uri` options.

Examples:

*Generate a gitignore template for simple rust projects and store it in a file
named `.gitignore`*

```bash
gitignore-template-generator rust > .gitignore
```

*Generate a gitignore template for python projects using a custom
generator (a fictive one here just for the example)*

```bash
gitignore-template-generator python \
  --server-url https://myapis.foobar.com
  --generator-uri /gitignore/generate
```

*List available templates*

```bash
gitignore-template-generator --list
```

## Development

Install `cargo-make`, dev tools and build the package:

```bash
./bootstrap.sh
```

Run the tests:

```bash
cargo test # unit + isolation + integration tests
cargo test --lib # unit tests
cargo test --test isolation_tests # isolation tests
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
