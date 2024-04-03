pub mod compound;
pub mod macros;
pub mod name;
pub mod ops;
pub mod units;

#[inline]
pub fn unit_symbol<T, U: name::NamedUnit<T>>(_: &U) -> String {
    U::symbol()
}

#[inline]
pub fn unit_name<T, U: name::NamedUnit<T>>(_: &U) -> String {
    U::unit_name()
}
