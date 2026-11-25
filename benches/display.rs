#![allow(missing_docs)]

use std::{fmt, hint::black_box};

struct ByteSizeAlwaysPad(bytesize::ByteSize);

impl fmt::Display for ByteSizeAlwaysPad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.0.display().to_string())
    }
}

#[divan::bench(sample_size = 32)]
fn display_inner_display() {
    black_box(format!(
        "{}",
        black_box(bytesize::ByteSize::kib(42).display()),
    ));
}

#[divan::bench(sample_size = 32)]
fn display_bytesize_standard() {
    black_box(format!("{}", black_box(bytesize::ByteSize::kib(42))));
}

#[divan::bench(sample_size = 16)]
fn display_bytesize_custom() {
    black_box(format!("|{:-^10}|", black_box(bytesize::ByteSize::kib(42))));
}

#[divan::bench(sample_size = 16)]
fn display_always_pad_standard() {
    black_box(format!(
        "{}",
        black_box(ByteSizeAlwaysPad(bytesize::ByteSize::kib(42))),
    ));
}

#[divan::bench(sample_size = 16)]
fn display_always_pad_custom() {
    black_box(format!(
        "|{:-^10}|",
        black_box(ByteSizeAlwaysPad(bytesize::ByteSize::kib(42))),
    ));
}

fn main() {
    divan::main();
}
