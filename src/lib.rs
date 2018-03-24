//! ByteSize is an utility that easily makes bytes size representation
//! and helps its arithmetic operations.
//!
//! ## Example
//!
//! ```ignore
//! extern crate bytesize;
//!
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
//! ```ignore
//!  assert_eq!("482 GiB".to_string(), ByteSize::gb(518).to_string(true));
//!  assert_eq!("518 GB".to_string(), ByteSize::gb(518).to_string(false));
//! ```

use std::fmt::{Display,Formatter,Result};
use std::ops::{Add,Sub,Mul,Div};


/// byte size for 1 byte
pub static B: u64 = 1;
/// bytes size for 1 kilobyte
pub static KB: u64 = 1000;
/// bytes size for 1 megabyte
pub static MB: u64 = 1000000;
/// bytes size for 1 gigabyte
pub static GB: u64 = 1000000000;
/// bytes size for 1 terabyte
pub static TB: u64 = 1000000000000;
/// bytes size for 1 petabyte
pub static PB: u64 = 1000000000000000;

/// bytes size for 1 kibibyte
pub static KIB: u64 = 1024;
/// bytes size for 1 mebibyte
pub static MIB: u64 = 1048576;
/// bytes size for 1 gibibyte
pub static GIB: u64 = 1073741824;
/// bytes size for 1 tebibyte
pub static TIB: u64 = 1099511627776;
/// bytes size for 1 pebibyte
pub static PIB: u64 = 1125899906842624;

static UNITS:    &'static str = "KMGTPE";
static UNITS_SI: &'static str = "kMGTPE";
static LN_KB:  f64 = 6.931471806; // ln 1024
static LN_KIB: f64 = 6.907755279; // ln 1000

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
/// Byte size representation
pub struct ByteSize {
  size: u64
}

impl ByteSize {
  #[inline(always)]
  pub fn b(size: u64) -> ByteSize {
    ByteSize {size: size}
  }

  #[inline(always)]
  pub fn kb(size: u64) -> ByteSize {
    ByteSize {size: size * KB}
  }

  #[inline(always)]
  pub fn kib(size: u64) -> ByteSize {
   ByteSize {size: size * KIB}
  }

  #[inline(always)]
  pub fn mb(size: u64) -> ByteSize {
    ByteSize {size: size * MB}
  }

  #[inline(always)]
  pub fn mib(size: u64) -> ByteSize {
    ByteSize {size: size * MIB}
  }

  #[inline(always)]
  pub fn gb(size: u64) -> ByteSize {
    ByteSize {size: size * GB}
  }

  #[inline(always)]
  pub fn gib(size: u64) -> ByteSize {
    ByteSize {size: size * GIB}
  }

  #[inline(always)]
  pub fn tb(size: u64) -> ByteSize {
    ByteSize {size: size * TB}
  }

  #[inline(always)]
  pub fn tib(size: u64) -> ByteSize {
    ByteSize {size: size * TIB}
  }

  #[inline(always)]
  pub fn pb(size: u64) -> ByteSize {
    ByteSize {size: size * PB}
  }

  #[inline(always)]
  pub fn pib(size: u64) -> ByteSize {
    ByteSize {size: size * PIB}
  }

  #[inline(always)]
  pub fn as_u64(&self) -> u64 {
    self.size
  }

  pub fn to_string(&self, si: bool) -> String {

    let unit = if si { KIB } else { KB };
    let unit_base =  if si { LN_KIB } else { LN_KB };
    let unit_prefix = if si { UNITS_SI.as_bytes() } else { UNITS.as_bytes() };
    let unit_suffix = if si { "iB" } else { "B" };

    if self.size < unit {
      format!("{} B", self.size)

    } else {
      let exp = match ((self.size as f64).ln() / unit_base) as usize {
        e if e == 0 => 1,
        e => e
      };

      format!("{} {}{}", (self.size / unit.pow(exp as u32)),
        unit_prefix[exp - 1] as char, unit_suffix)
    }
  }
}

impl Display for ByteSize {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.to_string(false))
  }
}

impl Add<ByteSize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn add(self, rhs: ByteSize) -> ByteSize {
    ByteSize {size: (self.size + rhs.size)}
  }
}

impl Sub<ByteSize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn sub(self, rhs: ByteSize) -> ByteSize {
    ByteSize {size: (self.size - rhs.size)}
  }
}

impl Mul<u64> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn mul(self, rhs: u64) -> ByteSize {
    ByteSize {size: (self.size * rhs)}
  }
}

impl Mul<ByteSize> for u64 {
  type Output = ByteSize;

  #[inline(always)]
  fn mul(self, rhs: ByteSize) -> ByteSize {
    rhs * self
  }
}

impl Div<u64> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn div(self, rhs: u64) -> ByteSize {
    ByteSize {size: (self.size / rhs)}
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_arithmetic() {
    let x = ByteSize::mb(1);
    let y = ByteSize::kb(100);

    assert_eq!(
      (x + y).as_u64(),
      1_100_000
    );

    assert_eq!(
      (x - y).as_u64(),
      900_000
    );

    assert_eq!(
      (7*x).as_u64(),
      7_000_000
    );
    assert_eq!(
      (x*42).as_u64(),
      42_000_000
    );

    assert_eq!(
      (x/100).as_u64(),
      10_000
    );
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
    assert_display("1 KB", ByteSize::kb(1));
    assert_display("301 KB", ByteSize::kb(301));
    assert_display("419 MB", ByteSize::mb(419));
    assert_display("518 GB", ByteSize::gb(518));
    assert_display("815 TB", ByteSize::tb(815));
    assert_display("609 PB", ByteSize::pb(609));
  }

  fn assert_to_string(expected: &str, b: ByteSize, si: bool) {
    assert_eq!(expected.to_string(), b.to_string(si));
  }

  #[test]
  fn test_to_string() {
    assert_to_string("215 B", ByteSize::b(215), true);
    assert_to_string("215 B", ByteSize::b(215), false);

    assert_to_string("1 kiB", ByteSize::kib(1), true);
    assert_to_string("1 KB", ByteSize::kib(1), false);

    assert_to_string("293 kiB", ByteSize::kb(301), true);
    assert_to_string("301 KB", ByteSize::kb(301), false);

    assert_to_string("1 MiB", ByteSize::mib(1), true);
    assert_to_string("1048 KB", ByteSize::mib(1), false);

    assert_to_string("399 MiB", ByteSize::mb(419), true);
    assert_to_string("419 MB", ByteSize::mb(419), false);

    assert_to_string("482 GiB", ByteSize::gb(518), true);
    assert_to_string("518 GB", ByteSize::gb(518), false);

    assert_to_string("741 TiB", ByteSize::tb(815), true);
    assert_to_string("815 TB", ByteSize::tb(815), false);

    assert_to_string("540 PiB", ByteSize::pb(609), true);
    assert_to_string("609 PB", ByteSize::pb(609), false);
  }

  #[test]
  fn test_default() {
    assert_eq!(ByteSize::b(0), ByteSize::default());
  }
}
