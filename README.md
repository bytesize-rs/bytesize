## ByteSize

<!-- prettier-ignore-start -->

[![CI](https://github.com/bytesize-rs/bytesize/actions/workflows/ci.yml/badge.svg)](https://github.com/bytesize-rs/bytesize/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/bytesize.svg)](https://crates.io/crates/bytesize)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

`ByteSize` is a semantic wrapper for byte count representations.

Features:

- Pre-defined constants for various size units (e.g., B, Kb, Kib, Mb, Mib, Gb, Gib, ... PB).
- `ByteSize` type which presents size units convertible to different size units.
- Arithmetic operations for `ByteSize`.
- `FromStr` impl for `ByteSize`, allowing for parsing string size representations like "1.5KiB" and "521TiB".
- Serde support for binary and human-readable deserializers like JSON.

### Examples

Construction using SI or IEC helpers.

```rust
use bytesize::ByteSize;

assert!(ByteSize::kib(4) > ByteSize::kb(4));
```

Display as human-readable string.

```rust
use bytesize::ByteSize;

assert_eq!("482.4 GiB", ByteSize::gb(518).to_string_as(true));
assert_eq!("518.0 GB", ByteSize::gb(518).to_string_as(false));
```

Arithmetic operations are supported.

```rust
use bytesize::ByteSize;

let plus = ByteSize::mb(1) + ByteSize::kb(100);
println!("{plus}");

let minus = ByteSize::tb(1) - ByteSize::gb(4);
assert_eq!(ByteSize::gb(996), minus);
```

<!-- cargo-rdme end -->
