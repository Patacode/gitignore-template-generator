# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[comment]: <> (@PlannedForNextRelease)
## [@Unreleased] - @ReleaseDate

### 🏗️ Build

- Add tasks to deploy and bump new version on trello
- Update version in .version file in pre release replacement hooks
- Add task to archive all Done trello cards in History on release
- Add task to deploy package with bump type give as cli arg

### ⚙️ Miscellaneous Tasks

- Add task input to select bump type
- Add new hidden file to store package version

## [0.3.8] - 2025-04-24 <a id="0.3.8"></a>

### 🚀 Features

- Create new method on ArgsParser to parse cli args and return result
- Add new function to parse cli args in unit tests
- Add new function to create vector of String

### 🐛 Bug Fixes

- Add missing newline before changelog content token in pre-release hooks

### 🚜 Refactor

- Replace helper module used by integration tests with the one of lib crate
- Implement custom logic for help and version options
- Move base error handling to parse method as before

### 🧪 Testing

- Add unit tests for all basic cli options (ie version, help, author)
- Add new test cases on other available cli options (ie pos args, server url)

### ⚙️ Miscellaneous Tasks

- Remove commit scope indicator in auto-generated changelogs
- Add new helper scope in allowed scope list
- Add new config scope to allowed scope list

## [0.3.7] - 2025-04-20 <a id="0.3.7"></a>

### ⚡ Performance

- Run pkg installations in dev task in parallel

### 🏗️ Build

- Remove git-commit task to avoid concurrency issues
- *(make)* Install git-cliff as part of dev task
- *(make)* Add git-cliff task to Makefile to generate changelog file
- *(make)* Add new tasks to bump generated changelog in CHANGELOG.md
- *(cargo)* Add special token in CHANGELOG.md when pre-release hooks

### ⚙️ Miscellaneous Tasks

- *(vscode)* Add changelog scope to allowed list
- *(vscode)* Add vscode scope to allowed list
- *(Makefile)* Update commit message bound to git-commit-changelog task
- *(vscode)* Add Makefile scope in allowed list
- *(vscode)* Add new words in allowed list
- *(cliff)* Add basic config file for git-cliff
- *(vscode)* Add 'cliff' scope in allowed scope list
- *(vscode)* Add make scope in allowed list
- *(cliff)* Skip style code format commit and remove header, footer and version in body from generated changelog
- *(vscode)* Add new task to generate changelog
- *(vscode)* Add cargo scope in allowed list

## [0.3.6] - 2025-04-20 <a id="0.3.6"></a>

### Changed

- help message of pos args and server url options
- refactor of integration tests to use cli option name constants and to refer
to error message constants through parent module

## [0.3.5] - 2025-04-20 <a id="0.3.5"></a>

### Added

- struct to wrap cli args parsing logic

### Changed

- refactoring of lib and bin crates

## [0.3.4] - 2025-04-19 <a id="0.3.4"></a>

### Added

- helper module for integration and isolation tests
- constant module to store globally shared constant fields

### Changed

- integration and isolation tests refactored

### Removed

- fmt dependency in fmt-commit task

### Fixed

- only CHANGELOG.md is commit and pushed in task fmt-commit

## [0.3.3] - 2025-04-19 <a id="0.3.3"></a>

### Changed

- structure of Makefile.toml

## [0.3.2] - 2025-04-18 <a id="0.3.2"></a>

### Added

- task in Makefile to commit and push changelog updates
- task in Makefile to format without commit
- vscode tasks to run Makefile tasks

### Changed

- logical grouping of tests in all remaining modules
- module happy and unhappy in integration tests renamed to success and failure
- module happy and unhappy in isolation tests renamed to success and failure

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
