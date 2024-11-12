# Changelog

[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Patchify]:            https://crates.io/crates/patchify
[Reqwest]:             https://crates.io/crates/reqwest
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].

## 0.1.1 (12 November 2024)

### Fixed

  - Reverted `std_process::FakeCommand` to the [Patchify][] approach and removed
    use of `Deref`

### Changed

  - Updated lint configuration for Rust 1.82
  - Updated crate dependencies


## 0.1.0 (11 September 2024)

### Added

  - Added `reqwest` module to mock [Reqwest][]
      - Added `reqwest::MockClient`
      - Added `reqwest::MockError`
      - Added `reqwest::MockRequestBuilder`
      - Added `reqwest::MockResponse`
      - Added `reqwest::create_mock_client()`
      - Added `reqwest::create_mock_response()`
  - Added `std_process` module to mock the Rust standard library's [`process`](https://doc.rust-lang.org/std/process/)
    module
      - Added `std_process::FakeCommand`
      - Added `std_process::MockCommand`
      - Added `std_process::MockStdio`
      - Added `std_process::mock_exit()`
  - Added feature flags for each module
  - Added README documentation

