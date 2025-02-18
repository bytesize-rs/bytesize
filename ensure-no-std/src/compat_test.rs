use alloc::string::ToString as _;

use bytesize::ByteSize;

pub fn create_byte_size() {
    ByteSize::kib(44).to_string();
}
