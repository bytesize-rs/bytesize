#![allow(missing_docs)]

use std::hint::black_box;

use bytesize::ByteSize;

#[divan::bench(args = ["4096", "42 MB", "42 MiB", "1.5 GiB"], sample_size = 2048)]
fn parse_bytesize(input: &str) -> ByteSize {
    black_box(input).parse().unwrap()
}

#[divan::bench(sample_size = 128)]
fn parse_unknown_unit() -> String {
    black_box("42 widgets").parse::<ByteSize>().unwrap_err()
}

fn main() {
    divan::main();
}
