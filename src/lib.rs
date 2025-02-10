//! `ByteSize` is a semantic wrapper for byte count representations.
//!
//! Features:
//!
//! - Pre-defined constants for various size units (e.g., B, Kb, Kib, Mb, Mib, Gb, Gib, ... PB).
//! - `ByteSize` type which presents size units convertible to different size units.
//! - Arithmetic operations for `ByteSize`.
//! - `FromStr` impl for `ByteSize`, allowing for parsing string size representations like "1.5KiB"
//!   and "521TiB".
//! - Serde support for binary and human-readable deserializers like JSON.
//!
//! # Examples
//!
//! Construction using SI or IEC helpers.
//!
//! ```
//! use bytesize::ByteSize;
//!
//! assert!(ByteSize::kib(4) > ByteSize::kb(4));
//! ```
//!
//! Display as human-readable string.
//!
//! ```
//! use bytesize::ByteSize;
//!
//! assert_eq!("482.4 GiB", ByteSize::gb(518).to_string_as(true));
//! assert_eq!("518.0 GB", ByteSize::gb(518).to_string_as(false));
//! ```
//!
//! Arithmetic operations are supported.
//!
//! ```
//! use bytesize::ByteSize;
//!
//! let plus = ByteSize::mb(1) + ByteSize::kb(100);
//! println!("{plus}");
//!
//! let minus = ByteSize::tb(1) - ByteSize::gb(4);
//! assert_eq!(ByteSize::gb(996), minus);
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

static UNITS: &str = "KMGTPE";
static UNITS_SI: &str = "KMGTPE";
static LN_KB: f64 = 6.931471806; // ln 1024
static LN_KIB: f64 = 6.907755279; // ln 1000

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
    let unit = if si_prefix { KIB } else { KB };
    let unit_base = if si_prefix { LN_KIB } else { LN_KB };
    let unit_prefix = if si_prefix {
        UNITS_SI.as_bytes()
    } else {
        UNITS.as_bytes()
    };
    let unit_suffix = if si_prefix { "iB" } else { "B" };

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
        f.pad(&to_string(self.0, true))
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

    fn assert_to_string(expected: &str, b: ByteSize, si: bool) {
        assert_eq!(expected.to_string(), b.to_string_as(si));
    }

    #[test]
    fn test_to_string_as() {
        assert_to_string("215 B", ByteSize::b(215), true);
        assert_to_string("215 B", ByteSize::b(215), false);

        assert_to_string("1.0 KiB", ByteSize::kib(1), true);
        assert_to_string("1.0 KB", ByteSize::kib(1), false);

        assert_to_string("293.9 KiB", ByteSize::kb(301), true);
        assert_to_string("301.0 KB", ByteSize::kb(301), false);

        assert_to_string("1.0 MiB", ByteSize::mib(1), true);
        assert_to_string("1048.6 KB", ByteSize::mib(1), false);

        // a bug case: https://github.com/flang-project/bytesize/issues/8
        assert_to_string("1.9 GiB", ByteSize::mib(1907), true);
        assert_to_string("2.0 GB", ByteSize::mib(1908), false);

        assert_to_string("399.6 MiB", ByteSize::mb(419), true);
        assert_to_string("419.0 MB", ByteSize::mb(419), false);

        assert_to_string("482.4 GiB", ByteSize::gb(518), true);
        assert_to_string("518.0 GB", ByteSize::gb(518), false);

        assert_to_string("741.2 TiB", ByteSize::tb(815), true);
        assert_to_string("815.0 TB", ByteSize::tb(815), false);

        assert_to_string("540.9 PiB", ByteSize::pb(609), true);
        assert_to_string("609.0 PB", ByteSize::pb(609), false);
    }

    #[test]
    fn test_default() {
        assert_eq!(ByteSize::b(0), ByteSize::default());
    }

    #[test]
    fn test_to_string() {
        assert_to_string("609.0 PB", ByteSize::pb(609), false);
    }
}
