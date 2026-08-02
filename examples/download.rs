//! Simulates a download with varying transfer speeds and displays its progress in bytes and speed in bits per second.

use std::{fmt, thread, time::Duration};

use bytesize::ByteSize;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

const FILE_SIZE: ByteSize = ByteSize::mb(100);
const CHUNK_SIZES: [ByteSize; 5] = [
    ByteSize::kb(500),
    ByteSize::kb(3_500),
    ByteSize::kb(1_000),
    ByteSize::kb(4_000),
    ByteSize::kb(1_000),
];
const DELAYS_MS: [u64; 5] = [180, 45, 140, 40, 110];

fn main() {
    println!("Simulating download...");

    let progress = ProgressBar::new(FILE_SIZE.as_u64());
    let style = ProgressStyle::with_template(
        "{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({speed}/s, {eta})",
    )
    .expect("valid progress template")
    .progress_chars("=> ")
    .with_key("speed", format_speed);

    progress.set_style(style);
    progress.set_message("Downloading");

    for _ in 0..10 {
        for (chunk_size, delay_ms) in CHUNK_SIZES.into_iter().zip(DELAYS_MS) {
            thread::sleep(Duration::from_millis(delay_ms));
            progress.inc(chunk_size.as_u64());
        }
    }

    progress.finish_with_message("Complete");
}

fn format_speed(state: &ProgressState, writer: &mut dyn fmt::Write) {
    let speed = ByteSize::b(state.per_sec() as u64).display().si_bits();
    let _ = write!(writer, "{speed}");
}
