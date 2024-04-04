//! # units
//!
//!

#![no_std]
#![forbid(clippy::all)]

extern crate alloc;

pub mod macros;
pub mod multipliers;
pub mod name;
pub mod ops;
pub mod units;
pub mod values;

/// Get symbol of a unit
#[inline]
pub fn unit_symbol<U: name::NamedUnit>() -> alloc::string::String {
    U::symbol()
}

/// Get name of a unit
#[inline]
pub fn unit_name<U: name::NamedUnit>() -> alloc::string::String {
    U::unit_name()
}

pub mod prelude {
    use super::*;

    pub use super::{unit_name, unit_symbol};
    pub use multipliers::*;
    pub use name::NamedUnit;
    pub use ops::{Div, Mul, Pow};
    pub use units::Unit;
    pub use values::Value;
}
