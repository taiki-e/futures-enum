# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

Releases may yanked if there is a security bug, a soundness bug, or a regression.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.1.18] - 2026-02-07

- Fix bug in crate name detection.

- Update `find-crate` to 0.7.

  This increases the minimum supported Rust version (MSRV) to Rust 1.76.

- Enable [release immutability](https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/immutable-releases).

## [0.1.17] - 2021-01-05

- Exclude unneeded files from crates.io.

## [0.1.16] - 2020-12-29

- Documentation improvements.

## [0.1.15] - 2020-11-06

- Update `derive_utils` to 0.11.

## [0.1.14] - 2020-09-07

- Update `find-crate` to 0.6.

## [0.1.13] - 2020-06-02

- Update `derive_utils` to 0.10.

## [0.1.12] - 2019-11-06

- Update to support `futures` 0.3.0.

## [0.1.11] - 2019-09-28

- Update to support `futures-preview` 0.3.0-alpha.19.

## [0.1.10] - 2019-08-14

- Add `renamed` feature to allow `find-crate` dependency to opt-out.

- Update `proc-macro2`, `syn`, and `quote` to 1.0.

- Update `derive_utils` to 0.9.

## [0.1.9] - 2019-07-04

- Update to support `futures-preview` 0.3.0-alpha.17.

- Update minimum `derive_utils` version to 0.8.0.

## [0.1.8] - 2019-06-16

- Add support for `futures::io::{AsyncSeek, AsyncBufRead}`.

- Update minimum `find-crate` version to 0.4.0.

## [0.1.7] - 2019-05-15

- Support for latest `futures` 0.3.0-alpha.16.

## [0.1.6] - 2019-04-16

- Update to new nightly.

- Update minimum `derive_utils` version to 0.7.0.

## [0.1.5] - 2019-03-04

- Update minimum `find-crate` version to 0.3.0.

- Fix documentation.

## [0.1.4] - 2019-02-21

- Update to new nightly.

## [0.1.3] - 2019-02-13

- `futures-enum` works well even if the dependency contains only sub-crates such as `futures-core`, `futures-util`, etc.

- Add generated code examples.

## [0.1.2] - 2019-02-05

- Update minimum `derive_utils` version to 0.6.3.

## [0.1.1] - 2019-02-03

- Documentation improvements.

## [0.1.0] - 2019-02-02

Initial release

[Unreleased]: https://github.com/taiki-e/futures-enum/compare/v0.1.18...HEAD
[0.1.18]: https://github.com/taiki-e/futures-enum/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/taiki-e/futures-enum/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/taiki-e/futures-enum/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/taiki-e/futures-enum/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/taiki-e/futures-enum/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/taiki-e/futures-enum/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/taiki-e/futures-enum/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/taiki-e/futures-enum/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/taiki-e/futures-enum/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/taiki-e/futures-enum/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/taiki-e/futures-enum/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/taiki-e/futures-enum/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/taiki-e/futures-enum/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/taiki-e/futures-enum/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/taiki-e/futures-enum/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/taiki-e/futures-enum/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/futures-enum/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/futures-enum/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/futures-enum/releases/tag/v0.1.0
