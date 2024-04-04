#![no_std]
extern crate alloc;

pub mod macros;
pub mod name;
pub mod ops;
pub mod units;
pub mod values;

#[inline]
pub fn unit_symbol<U: name::NamedUnit>(_: &U) -> alloc::string::String {
    U::symbol()
}

#[inline]
pub fn unit_name<U: name::NamedUnit>(_: &U) -> alloc::string::String {
    U::unit_name()
}
