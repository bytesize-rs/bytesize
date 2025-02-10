//! ByteSize is an utility that easily makes bytes size representation
//! and helps its arithmetic operations.
//!
//! ## Example
//!
//! ```ignore
//! use bytesize::ByteSize;
//!
//! fn byte_arithmetic_operator() {
//!   let x = ByteSize::mb(1);
//!   let y = ByteSize::kb(100);
//!
//!   let plus = x + y;
//!   print!("{} bytes", plus.as_u64());
//!
//!   let minus = ByteSize::tb(100) - ByteSize::gb(4);
//!   print!("{} bytes", minus.as_u64());
//! }
//! ```
//!
//! It also provides its human readable string as follows:
//!
//! ```
//! use bytesize::ByteSize;
//!
//! assert_eq!("482.4 GiB", ByteSize::gb(518).to_string_as(false));
//! assert_eq!("518.0 GB", ByteSize::gb(518).to_string_as(true));
//! ```

mod parse;
#[cfg(feature = "serde")]
mod serde;

use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// byte size for 1 byte
pub const B: u64 = 1;
/// bytes size for 1 kilobyte
pub const KB: u64 = 1_000;
/// bytes size for 1 megabyte
pub const MB: u64 = 1_000_000;
/// bytes size for 1 gigabyte
pub const GB: u64 = 1_000_000_000;
/// bytes size for 1 terabyte
pub const TB: u64 = 1_000_000_000_000;
/// bytes size for 1 petabyte
pub const PB: u64 = 1_000_000_000_000_000;

/// bytes size for 1 kibibyte
pub const KIB: u64 = 1_024;
/// bytes size for 1 mebibyte
pub const MIB: u64 = 1_048_576;
/// bytes size for 1 gibibyte
pub const GIB: u64 = 1_073_741_824;
/// bytes size for 1 tebibyte
pub const TIB: u64 = 1_099_511_627_776;
/// bytes size for 1 pebibyte
pub const PIB: u64 = 1_125_899_906_842_624;

/// IEC (binary) units.
///
/// See <https://en.wikipedia.org/wiki/Kilobyte>.
const UNITS_IEC: &str = "KMGTPE";

/// SI (decimal) units.
///
///
/// See <https://en.wikipedia.org/wiki/Kilobyte>.
const UNITS_SI: &str = "kMGTPE";

/// `ln(1024) ~= 6.931`
const LN_KIB: f64 = 6.931471806;

/// `ln(1000) ~= 6.908`
const LN_KB: f64 = 6.907755279;

pub fn kb<V: Into<u64>>(size: V) -> u64 {
    size.into() * KB
}

pub fn kib<V: Into<u64>>(size: V) -> u64 {
    size.into() * KIB
}

pub fn mb<V: Into<u64>>(size: V) -> u64 {
    size.into() * MB
}

pub fn mib<V: Into<u64>>(size: V) -> u64 {
    size.into() * MIB
}

pub fn gb<V: Into<u64>>(size: V) -> u64 {
    size.into() * GB
}

pub fn gib<V: Into<u64>>(size: V) -> u64 {
    size.into() * GIB
}

pub fn tb<V: Into<u64>>(size: V) -> u64 {
    size.into() * TB
}

pub fn tib<V: Into<u64>>(size: V) -> u64 {
    size.into() * TIB
}

pub fn pb<V: Into<u64>>(size: V) -> u64 {
    size.into() * PB
}

pub fn pib<V: Into<u64>>(size: V) -> u64 {
    size.into() * PIB
}

/// Byte size representation
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ByteSize(pub u64);

impl ByteSize {
    #[inline(always)]
    pub const fn b(size: u64) -> ByteSize {
        ByteSize(size)
    }

    #[inline(always)]
    pub const fn kb(size: u64) -> ByteSize {
        ByteSize(size * KB)
    }

    #[inline(always)]
    pub const fn kib(size: u64) -> ByteSize {
        ByteSize(size * KIB)
    }

    #[inline(always)]
    pub const fn mb(size: u64) -> ByteSize {
        ByteSize(size * MB)
    }

    #[inline(always)]
    pub const fn mib(size: u64) -> ByteSize {
        ByteSize(size * MIB)
    }

    #[inline(always)]
    pub const fn gb(size: u64) -> ByteSize {
        ByteSize(size * GB)
    }

    #[inline(always)]
    pub const fn gib(size: u64) -> ByteSize {
        ByteSize(size * GIB)
    }

    #[inline(always)]
    pub const fn tb(size: u64) -> ByteSize {
        ByteSize(size * TB)
    }

    #[inline(always)]
    pub const fn tib(size: u64) -> ByteSize {
        ByteSize(size * TIB)
    }

    #[inline(always)]
    pub const fn pb(size: u64) -> ByteSize {
        ByteSize(size * PB)
    }

    #[inline(always)]
    pub const fn pib(size: u64) -> ByteSize {
        ByteSize(size * PIB)
    }

    #[inline(always)]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn to_string_as(&self, si_unit: bool) -> String {
        to_string(self.0, si_unit)
    }
}

pub fn to_string(bytes: u64, si_prefix: bool) -> String {
    let unit = if si_prefix { KB } else { KIB };
    let unit_base = if si_prefix { LN_KB } else { LN_KIB };

    let unit_prefix = match si_prefix {
        true => UNITS_SI.as_bytes(),
        false => UNITS_IEC.as_bytes(),
    };
    let unit_suffix = match si_prefix {
        true => "B",
        false => "iB",
    };

    if bytes < unit {
        format!("{} B", bytes)
    } else {
        let size = bytes as f64;
        let exp = match (size.ln() / unit_base) as usize {
            0 => 1,
            e => e,
        };

        format!(
            "{:.1} {}{}",
            (size / unit.pow(exp as u32) as f64),
            unit_prefix[exp - 1] as char,
            unit_suffix
        )
    }
}

impl Display for ByteSize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(&to_string(self.0, false))
    }
}

impl Debug for ByteSize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

macro_rules! commutative_op {
    ($t:ty) => {
        impl Add<ByteSize> for $t {
            type Output = ByteSize;
            #[inline(always)]
            fn add(self, rhs: ByteSize) -> ByteSize {
                ByteSize(rhs.0 + (self as u64))
            }
        }

        impl Mul<ByteSize> for $t {
            type Output = ByteSize;
            #[inline(always)]
            fn mul(self, rhs: ByteSize) -> ByteSize {
                ByteSize(rhs.0 * (self as u64))
            }
        }
    };
}

commutative_op!(u64);
commutative_op!(u32);
commutative_op!(u16);
commutative_op!(u8);

impl Add<ByteSize> for ByteSize {
    type Output = ByteSize;

    #[inline(always)]
    fn add(self, rhs: ByteSize) -> ByteSize {
        ByteSize(self.0 + rhs.0)
    }
}

impl AddAssign<ByteSize> for ByteSize {
    #[inline(always)]
    fn add_assign(&mut self, rhs: ByteSize) {
        self.0 += rhs.0
    }
}

impl<T> Add<T> for ByteSize
where
    T: Into<u64>,
{
    type Output = ByteSize;
    #[inline(always)]
    fn add(self, rhs: T) -> ByteSize {
        ByteSize(self.0 + (rhs.into()))
    }
}

impl<T> AddAssign<T> for ByteSize
where
    T: Into<u64>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into();
    }
}

impl Sub<ByteSize> for ByteSize {
    type Output = ByteSize;

    #[inline(always)]
    fn sub(self, rhs: ByteSize) -> ByteSize {
        ByteSize(self.0 - rhs.0)
    }
}

impl SubAssign<ByteSize> for ByteSize {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: ByteSize) {
        self.0 -= rhs.0
    }
}

impl<T> Sub<T> for ByteSize
where
    T: Into<u64>,
{
    type Output = ByteSize;
    #[inline(always)]
    fn sub(self, rhs: T) -> ByteSize {
        ByteSize(self.0 - (rhs.into()))
    }
}

impl<T> SubAssign<T> for ByteSize
where
    T: Into<u64>,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs.into();
    }
}

impl<T> Mul<T> for ByteSize
where
    T: Into<u64>,
{
    type Output = ByteSize;
    #[inline(always)]
    fn mul(self, rhs: T) -> ByteSize {
        ByteSize(self.0 * rhs.into())
    }
}

impl<T> MulAssign<T> for ByteSize
where
    T: Into<u64>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_op() {
        let mut x = ByteSize::mb(1);
        let y = ByteSize::kb(100);

        assert_eq!((x + y).as_u64(), 1_100_000u64);

        assert_eq!((x - y).as_u64(), 900_000u64);

        assert_eq!((x + (100 * 1000) as u64).as_u64(), 1_100_000);

        assert_eq!((x * 2u64).as_u64(), 2_000_000);

        x += y;
        assert_eq!(x.as_u64(), 1_100_000);
        x *= 2u64;
        assert_eq!(x.as_u64(), 2_200_000);
    }

    #[allow(clippy::unnecessary_cast)]
    #[test]
    fn test_arithmetic_primitives() {
        let mut x = ByteSize::mb(1);

        assert_eq!((x + MB as u64).as_u64(), 2_000_000);

        assert_eq!((x + MB as u32).as_u64(), 2_000_000);

        assert_eq!((x + KB as u16).as_u64(), 1_001_000);

        assert_eq!((x + B as u8).as_u64(), 1_000_001);

        assert_eq!((x - MB as u64).as_u64(), 0);

        assert_eq!((x - MB as u32).as_u64(), 0);

        assert_eq!((x - KB as u32).as_u64(), 999_000);

        assert_eq!((x - B as u32).as_u64(), 999_999);

        x += MB as u64;
        x += MB as u32;
        x += 10u16;
        x += 1u8;
        assert_eq!(x.as_u64(), 3_000_011);
    }

    #[test]
    fn test_comparison() {
        assert!(ByteSize::mb(1) == ByteSize::kb(1000));
        assert!(ByteSize::mib(1) == ByteSize::kib(1024));
        assert!(ByteSize::mb(1) != ByteSize::kib(1024));
        assert!(ByteSize::mb(1) < ByteSize::kib(1024));
        assert!(ByteSize::b(0) < ByteSize::tib(1));
    }

    #[track_caller]
    fn assert_display(expected: &str, b: ByteSize) {
        assert_eq!(expected, format!("{}", b));
    }

    #[test]
    fn test_display() {
        assert_display("215 B", ByteSize::b(215));
        assert_display("1.0 KiB", ByteSize::kib(1));
        assert_display("301.0 KiB", ByteSize::kib(301));
        assert_display("419.0 MiB", ByteSize::mib(419));
        assert_display("518.0 GiB", ByteSize::gib(518));
        assert_display("815.0 TiB", ByteSize::tib(815));
        assert_display("609.0 PiB", ByteSize::pib(609));
    }

    #[test]
    fn test_display_alignment() {
        assert_eq!("|357 B     |", format!("|{:10}|", ByteSize(357)));
        assert_eq!("|     357 B|", format!("|{:>10}|", ByteSize(357)));
        assert_eq!("|357 B     |", format!("|{:<10}|", ByteSize(357)));
        assert_eq!("|  357 B   |", format!("|{:^10}|", ByteSize(357)));

        assert_eq!("|-----357 B|", format!("|{:->10}|", ByteSize(357)));
        assert_eq!("|357 B-----|", format!("|{:-<10}|", ByteSize(357)));
        assert_eq!("|--357 B---|", format!("|{:-^10}|", ByteSize(357)));
    }

    #[track_caller]
    fn assert_to_string(expected: &str, b: ByteSize, si: bool) {
        assert_eq!(expected.to_string(), b.to_string_as(si));
    }

    #[test]
    fn test_to_string_as() {
        assert_to_string("215 B", ByteSize::b(215), false);
        assert_to_string("215 B", ByteSize::b(215), true);

        assert_to_string("1.0 KiB", ByteSize::kib(1), false);
        assert_to_string("1.0 kB", ByteSize::kib(1), true);

        assert_to_string("293.9 KiB", ByteSize::kb(301), false);
        assert_to_string("301.0 kB", ByteSize::kb(301), true);

        assert_to_string("1024.0 KiB", ByteSize::mib(1), false);
        assert_to_string("1.0 MB", ByteSize::mib(1), true);

        assert_to_string("1.9 GiB", ByteSize::mib(1907), false);
        assert_to_string("2.0 GB", ByteSize::mib(1908), true);

        assert_to_string("399.6 MiB", ByteSize::mb(419), false);
        assert_to_string("419.0 MB", ByteSize::mb(419), true);

        assert_to_string("482.4 GiB", ByteSize::gb(518), false);
        assert_to_string("518.0 GB", ByteSize::gb(518), true);

        assert_to_string("741.2 TiB", ByteSize::tb(815), false);
        assert_to_string("815.0 TB", ByteSize::tb(815), true);

        assert_to_string("540.9 PiB", ByteSize::pb(609), false);
        assert_to_string("609.0 PB", ByteSize::pb(609), true);
    }

    #[test]
    fn test_default() {
        assert_eq!(ByteSize::b(0), ByteSize::default());
    }

    #[test]
    fn test_to_string() {
        assert_to_string("609.0 PB", ByteSize::pb(609), true);
    }
}
