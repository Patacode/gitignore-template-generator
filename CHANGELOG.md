# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[comment]: <> (@PlannedForNextRelease)
## [@Unreleased] - @ReleaseDate

### 🧪 Testing

- [benches] - Add benchmarking to compare template generation with/without robust check
- [benches] - Add benchmarking for list option

## [0.7.0] - 2025-05-11 <a id="0.7.0"></a>

### 🚀 Features

- [http_client] - Add new type of http client to mock response with endpoint mapping (*New struct 'MockEndpointHttpClient'*)
- [core] - Add new method to generate templates from api with robust template checks (*Same as 'generate_from_api' but will first fetch the list of available template names to existence check and will return a proper error accordingly*)
- [parser] - Add new check option to enable robust template name check
- Integrate new check option into main binary crate

## [0.6.2] - 2025-05-07 <a id="0.6.2"></a>

### 🚀 Features

- [core] - Print template list output items one per line

### 🚜 Refactor

- [core] - Wrap endpoint_uri in Option enum (*To have cleaner and more coherent api. Will default to same value as arg parser itself*)

## [0.6.1] - 2025-05-07 <a id="0.6.1"></a>

### ⚙️ Miscellaneous Tasks

- [readme] - Add missing mention of isolation tests and make examples more concise (*Remove example output to make them easily copyable*)
- [readme] - Update server url in examples (*Using a server url that does not exist*)
- [readme] - Update example descriptions

## [0.6.0] - 2025-05-07 <a id="0.6.0"></a>

### 🚀 Features

- Add new lister-uri option to set custom lister endpoint uri

### 🚜 Refactor

- Rename 'endpoint-uri' option to 'generator-uri' (*To differentiate from lister uri*)
- Update help message of generator-uri and server-uri options (*To be more concise, less redundant with context*)

### ⚙️ Miscellaneous Tasks

- [readme] - Improve with more examples and explanations

## [0.5.0] - 2025-05-06 <a id="0.5.0"></a>

- Same as version 0.4.9. The latter has been yanked due to versioning mix-up (*It was supposed to be a minor version*) 

## [0.4.9] - 2025-05-06 <a id="0.4.9"></a>

### 🚀 Features

- Add new 'list' option to list available templates using toptal api

### 🐛 Bug Fixes

- Use println! macro over print! in main binary crate (*To add systematic ending newline to success response*)
- [http_client] - Trim success body response (*To remove leading and trailing whitespaces*)

### 🚜 Refactor

- [core] - Rename 'GitignoreTemplateGenerator' to 'GitignoreTemplateManager' (*To use a more appropriate and generic naming as it will handle both generation and listing of templates*)
- [constant] - Rename 'template_generator' module to 'template_manager' module

### 📚 Documentation

- [core] - Update rdoc of 'GitignoreTemplateGenerator' (*To make it more generic and identify it as 'manager' as it will generate and list gitignore templates*)
- [parser] - Update ref to template_manager constants

## [0.4.8] - 2025-05-05 <a id="0.4.8"></a>

### 🚜 Refactor

- Move unhappy integration test in failure module

### ⚡ Performance

- [benches] - Move mock server creation out of iter function (*So create it only once, at the beginning of benchmark function*)

### 🧪 Testing

- Add benchmarking on binary crate
- [parser] - Add simple unit test on successful parsing by parse() method
- Readd previously deleted integration tests (*On version, author and help options + few error cases to ensure proper exit code and output to stdout/stderr*)

## [0.4.7] - 2025-04-29 <a id="0.4.7"></a>

### 🎨 Styling

- [cargo] - Sort dependencies by alphabetic order (*Use cargo-sort for that*)
- Sort and group imports using new strategies

### 🏗️ Build

- [make] - Remove echoing of url encoded card name in rename-trello-cards task
- [make] - Include cargo-sort installation in dev task
- [make] - Enable nightly features in fmt tasks

## [0.4.6] - 2025-04-28 <a id="0.4.6"></a>

### 🏗️ Build

- [make] - Add task to rename trello cards with version specifier (*Trigger on release, before archiving cards*)

## [0.4.5] - 2025-04-28 <a id="0.4.5"></a>

### 🚀 Features

- [validator] - Add new validator method to check for whitespaces
- Shorten help text of endpoint uri cli option (*To be more concise*)

### 🚜 Refactor

- [validator] - Remove needless borrow

## [0.4.4] - 2025-04-28 <a id="0.4.4"></a>

### 🚀 Features

- [http_client] - Add global timeout of 5s when making get http call (*To prevent long waiting time*)

### 🚜 Refactor

- [http_client] - Add 'Http' before 'Client' in UreqClient and MockClient (*To align on same naming convention everywhere*)

### 🧪 Testing

- Add integration test on server not found error (*Might occur when DNS lookup fails with provided server url*)
- Add isolation tests to cover new cases (*One to test server url and endpoint uri in successful cases and one to test an undefined endpoint uri*)

### 🏗️ Build

- [make] - Add task to build doc

### ⚙️ Miscellaneous Tasks

- [vscode] - Add task to trigger doc make task

## [0.4.3] - 2025-04-28 <a id="0.4.3"></a>

### 🏗️ Build

- [cliff] - Include commit scope and body if present in generated changelog
- [cliff] - Remove release tag commits from generated changelog

## [0.4.2] - 2025-04-28 <a id="0.4.2"></a>

### 🐛 Bug Fixes

- Include missing assets in package

## [0.4.1] - 2025-04-28 <a id="0.4.1"></a>

### 🚜 Refactor

- Move parser definition and creation logic to 'constructor'
- Rename DefaultArgsParser to ClapArgsParser
- Split cli arg definitions in separate modules

### 📚 Documentation

- Improve doc of Args struct fields and ArgsParser trait

### ⚙️ Miscellaneous Tasks

- Correct typo in usage section

## [0.4.0] - 2025-04-27 <a id="0.4.0"></a>

### 🚀 Features

- Add new cli option endpoint-uri to set custom template generator service
endpoint uri

### 🚜 Refactor

- Remove unnecessary comments

## [0.3.12] - 2025-04-27 <a id="0.3.12"></a>

### 🚜 Refactor

- Remove unused regex module
- Remove unused REGEX_NO_MATCH error message constant
- Remove config module and replace it with parser module entirely
- Rename ProgramError struct to ProgramExit
- Rename error_kind field to kind
- Rename ErrorKind enum to ExitKind
- Rename 'Other' ExitKind enum field to Error

### 📚 Documentation

- Add doc for all modules

### ⚙️ Miscellaneous Tasks

- Update help output in README to match current one
- Improve usage section in README

## [0.3.11] - 2025-04-27 <a id="0.3.11"></a>

### 🚜 Refactor

- Move ProgramError struct and ErrorKind enum to core module
- Replace &str to string slice &[String] for template names param
- Use static dispatch instead of dynamic dispatch for http_client param
- Change type of error_kind field from Option<ErrorKind> to ErrorKind
- Extract ArgsParser trait and underlying implementation to new parser module

### ⚙️ Miscellaneous Tasks

- Add changelog for previous version

## [0.3.10] - 2025-04-27 <a id="0.3.10"></a>

### 🐛 Bug Fixes

- Remove trello dumping in dryrun release task

### 🚜 Refactor

- Remove integration tests already covered by unit tests

### ⚙️ Miscellaneous Tasks

- Update deploy task to now release with given bump level input

## [0.3.9] - 2025-04-24 <a id="0.3.9"></a>

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
