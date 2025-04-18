# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[comment]: <> (@PlannedForNextRelease)
## [@Unreleased] - @ReleaseDate

## [0.3.1] - 2025-04-18 <a id="0.3.1"></a>

### Added

- more integration tests for extended coverage
- isolation tests to cover edge cases
- option to change server url
- tasks in Makefile to format and lint project
- task in Makefile to run all type of tests at once

### Changed

- logical grouping some tests in some module

## [0.3.0] - 2025-04-18 <a id="0.3.0"></a>

### Added

- Unit and integration tests

### Changed

- Usage example in README include output redirection to file
- Internal structure of library and binary crate
- Usage of struct for core functions access

## [0.2.1] - 2025-04-16 <a id="0.2.1"></a>

### Added

- keywords and categories in Cargo.toml for better searchability using
crates.io search bar

### Changed

- About message displayed in help message

## [0.2.0] - 2025-04-16 <a id="0.2.0"></a>

### Changed

- clap library to implement a more robust CLI args handling
- Each CLI template arg must be given as a separate value, so ',' char is now
prohibited from provided values
- Split of functionalities in separate modules, following facade pattern
- Description of binary crate in README file, github repo and Cargo.toml
- Enhanced usage and development section in README

## [0.1.0] - 2025-04-09 <a id="0.1.0"></a>

### Added

- First binary crate release
- Basic functionalities to generate gitignore templates
using toptal API
