# Changelog

## Unreleased

- Use IEC (binary) format by default with `Display`.
- Use "kB" for SI unit.
- Implement `Sub<ByteSize>` for `ByteSize`.
- Implement `Sub<impl Into<u64>>` for `ByteSize`.
- Implement `SubAssign<ByteSize>` for `ByteSize`.
- Implement `SubAssign<impl Into<u64>>` for `ByteSize`.
- Reject parsing non-unit characters after whitespace.
- Remove `ByteSize::to_string_as()` method.
- Remove top-level `to_string()` method.
