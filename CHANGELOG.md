# Changelog

## Unreleased

## [2.2.0](https://github.com/bytesize-rs/bytesize/compare/v2.1.0...v2.2.0) - 2025-11-10

### Added

- add custom unit conversions ([#111](https://github.com/bytesize-rs/bytesize/pull/111))

### Other

- fix release scripts
- fix release branch ref
- add release-plz
- *(deps)* bump codecov/codecov-action from 5.5.0 to 5.5.1 ([#116](https://github.com/bytesize-rs/bytesize/pull/116))
- *(deps)* bump actions-rust-lang/setup-rust-toolchain ([#119](https://github.com/bytesize-rs/bytesize/pull/119))
- *(deps)* bump taiki-e/install-action from 2.58.29 to 2.62.43 ([#120](https://github.com/bytesize-rs/bytesize/pull/120))

- Add `ByteSize::as_*()` methods to return equivalent sizes in KB, GiB, etc.

## 2.1.0

- Support parsing and formatting exabytes (EB) & exbibytes (EiB).
- Migrate `serde` dependency to `serde_core`.

## 2.0.1

- Add support for precision in `Display` implementations.

## v2.0.0

- Add support for `no_std` targets.
- Use IEC (binary) format by default with `Display`.
- Use "kB" for SI unit.
- Add `Display` type for customizing printed format.
- Add `ByteSize::display()` method.
- Implement `Sub<ByteSize>` for `ByteSize`.
- Implement `Sub<impl Into<u64>>` for `ByteSize`.
- Implement `SubAssign<ByteSize>` for `ByteSize`.
- Implement `SubAssign<impl Into<u64>>` for `ByteSize`.
- Reject parsing non-unit characters after whitespace.
- Remove `ByteSize::to_string_as()` method.
- Remove top-level `to_string()` method.
- Remove top-level `B` constant.
