# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.3] - 2025-05-08

### Changed
- Modified the `UDEPS_CMD` constant to relax the usage restrictions. The program no longer requires nightly rustc to be the default.
- Updated the `check_rust_nightly` function to execute nightly rustc via rustup.

### Fixed
- Fixed Chinese translation in error messages within the `execute_udeps` function.

## [0.6.2] - 2025-05-04

### Changed
- Improved error handling with exit code validation in `run_clippy()` and `run_cargo_deny()`

## [0.6.1] - 2025-05-03

### Fixed
- Fixed CLI version display to automatically sync with Cargo.toml version using `env!("CARGO_PKG_VERSION")`

## [0.6.0] - 2025-05-03

### Added
- Added depcheck to full-check workflow (clippy -> depcheck -> deny -> build)

## [0.5.0] - 2025-05-03

### Added
- Added `--clippy` flag to run clippy lint checks
- Added `--deny` flag to run cargo-deny checks
- Added `--full-check` flag to run complete workflow (clippy -> deny -> build)

## [0.4.0] - 2025-05-02

### Changed
- Translated all runtime Chinese outputs to user-friendly English

## [0.3.0] - 2025-05-02

### Fixed
- Fixed std library build parameters in `BuildSystem::build` function to ensure proper compilation
