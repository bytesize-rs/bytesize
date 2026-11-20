use core::fmt::{self, Write as _};

use bytesize::{ByteSize, ByteSizeParseError, Unit, UnitParseError, KIB};

pub fn check() {
    assert_error::<ByteSizeParseError>();
    assert_error::<UnitParseError>();

    let size = "44 KiB".parse::<ByteSize>().unwrap();
    let bytes = size.as_u64();

    assert!(bytes == KIB * 44);
    assert!("KiB".parse::<Unit>().unwrap() * 44 == bytes);

    let mut output = Buffer::new();
    write!(&mut output, "|{size:>13.5}|").unwrap();
    assert!(output.as_str() == "| 44.00000 KiB|");
}

fn assert_error<E: core::error::Error>() {}

struct Buffer {
    bytes: [u8; 32],
    len: usize,
}

impl Buffer {
    const fn new() -> Self {
        Self {
            bytes: [0; 32],
            len: 0,
        }
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap()
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let end = self.len.checked_add(value.len()).ok_or(fmt::Error)?;
        let target = self.bytes.get_mut(self.len..end).ok_or(fmt::Error)?;
        target.copy_from_slice(value.as_bytes());
        self.len = end;
        Ok(())
    }
}
