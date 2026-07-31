use bytesize::{ByteSize, Unit, KIB};

pub fn create_byte_size() {
    let size = ByteSize::kib(44);
    let bytes = size.as_u64();

    assert!(bytes == KIB * 44);
    assert!(Unit::KibiByte * 44 == bytes);
}
