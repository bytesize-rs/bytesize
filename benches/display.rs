#![allow(missing_docs)]

use std::{fmt, hint::black_box};

struct ByteSizeAlwaysPad(bytesize::ByteSize);

impl fmt::Display for ByteSizeAlwaysPad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.0.display().to_string())
    }
}

#[divan::bench(sample_size = 32)]
fn display_inner_display() -> String {
    black_box(bytesize::ByteSize::kib(42).display()).to_string()
}

#[divan::bench(sample_size = 32)]
fn display_bytesize_standard() -> String {
    black_box(bytesize::ByteSize::kib(42)).to_string()
}

#[divan::bench(sample_size = 16)]
fn display_bytesize_custom() -> String {
    format!("|{:-^10}|", black_box(bytesize::ByteSize::kib(42)))
}

#[divan::bench(sample_size = 16)]
fn display_always_pad_standard() -> String {
    black_box(ByteSizeAlwaysPad(bytesize::ByteSize::kib(42))).to_string()
}

#[divan::bench(sample_size = 16)]
fn display_always_pad_custom() -> String {
    format!(
        "|{:-^10}|",
        black_box(ByteSizeAlwaysPad(bytesize::ByteSize::kib(42))),
    )
}

#[cfg(feature = "serde")]
#[divan::bench(sample_size = 32)]
fn serialize_json() -> String {
    serde_json::to_string(black_box(&bytesize::ByteSize::kib(42))).unwrap()
}

#[cfg(feature = "serde")]
#[divan::bench(sample_size = 32)]
fn serialize_cbor() -> Vec<u8> {
    let mut output = Vec::new();
    ciborium::into_writer(black_box(&bytesize::ByteSize::kib(42)), &mut output).unwrap();
    output
}

fn main() {
    divan::main();
}
