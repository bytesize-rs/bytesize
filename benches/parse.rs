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

#[cfg(feature = "serde")]
#[divan::bench(args = ["43008", r#""42 KiB""#], sample_size = 128)]
fn deserialize_json(input: &str) -> ByteSize {
    serde_json::from_str(black_box(input)).unwrap()
}

#[cfg(feature = "serde")]
#[divan::bench(sample_size = 128)]
fn deserialize_cbor(bencher: divan::Bencher<'_, '_>) {
    let mut input = Vec::new();
    ciborium::into_writer(&ByteSize::kib(42), &mut input).unwrap();

    bencher.bench(|| ciborium::from_reader::<ByteSize, _>(black_box(input.as_slice())).unwrap());
}

fn main() {
    divan::main();
}
