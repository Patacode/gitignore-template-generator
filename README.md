# gitignore-template-generator

[<img alt="github" src="https://img.shields.io/badge/github-black?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Patacode/gitignore-template-generator)
[<img alt="crates.io" src="https://img.shields.io/crates/v/gitignore-template-generator?logoColor=E3A835&style=for-the-badge&color=9c7325&logo=rust" height="20">](https://crates.io/crates/gitignore-template-generator)
[<img alt="crates.io" src="https://img.shields.io/crates/d/gitignore-template-generator?logoColor=E3A835&style=for-the-badge&color=152673" height="20">](https://crates.io/crates/gitignore-template-generator)
<br/>
[<img alt="documentation" src="https://img.shields.io/badge/Documentation-blue?style=for-the-badge&logo=rust" height="20">](https://docs.rs/gitignore-template-generator/0.14.1/gitignore_template_generator)

A binary crate to generate templates for .gitignore files

## Installation

Current release: [0.14.1](CHANGELOG.md#0.14.1)

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
  -c, --check                          Enable robust template names check
  -g, --generator-uri <GENERATOR_URI>  The template generator uri [default: /developers/gitignore/api]
  -l, --list                           List available templates
  -i, --lister-uri <LISTER_URI>        The template lister uri [default: /developers/gitignore/api/list]
  -s, --server-url <SERVER_URL>        The template manager url [default: https://www.toptal.com]
  -t, --timeout <TIMEOUT>              The template generation and listing service calls timeout [default: 5s/5000ms]
  -u, --timeout-unit <TIMEOUT_UNIT>    The timeout unit [default: second] [possible values: millisecond, second]
  -h, --help                           Print help
  -V, --version                        Print version
  -a, --author                         Print author

Version: 0.14.1
Author: Patacode <pata.codegineer@gmail.com>
```

By default, this tool is a simple API binder to `toptal` gitignore template
generation service. It takes gitignore template names as positional arguments
and generates a gitignore template for you.

It supports a variety of CLI options and feature flags to
customize its behavior with finer control. Have a look at the [official crate documentation](https://docs.rs/gitignore-template-generator/0.14.1/gitignore_template_generator)
for more details.

You might be interested in the [local templating](https://docs.rs/gitignore-template-generator/0.14.1/gitignore_template_generator/#local-templating)
feature for example, which allows you to generate gitignore templates from your
local file system.

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
  --exclude-files src/lib.rs src/main.rs benches/* tests/* src/constant.rs src/core.rs src/http_client.rs src/parser.rs src/validator.rs src/**/tests.rs src/**/api.rs \
  --output-dir target/tarpaulin
```

Benchmark the binary crate:

```bash
cargo bench
```

Generate and open documentation:

```bash
cargo doc --open
```

Lint code:

```bash
cargo clippy --all-targets --all-features
```

Format code with nightly features (required to sort imports):

```bash
cargo +nightly fmt -- --unstable-features
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
