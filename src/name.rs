use crate::{compound::CompoundUnit, ops::{Div, Mul}, units::Unit};

pub trait NamedUnit<T>: Unit<T> {
    fn unit_name() -> String;
    fn symbol() -> String;
}

impl<T: Copy, L: Unit<T> + NamedUnit<T>, R: Unit<T> + NamedUnit<T>> NamedUnit<T> for Mul<T, L, R> {
    fn symbol() -> String {
        format!("{}•{}", L::symbol(), R::symbol())
    }

    fn unit_name() -> String {
        format!("{}•{}", L::unit_name(), R::symbol())
    }
}

impl<T: Copy, L: Unit<T> + NamedUnit<T>, R: Unit<T> + NamedUnit<T>> NamedUnit<T> for Div<T, L, R> {
    fn symbol() -> String {
        format!("{}•{}⁻¹", L::symbol(), R::symbol())
    }

    fn unit_name() -> String {
        format!("{}•{}⁻¹", L::unit_name(), R::symbol())
    }
}

impl<T: Copy, U: NamedUnit<T>> NamedUnit<T> for CompoundUnit<T, U> {
    #[inline]
    fn symbol() -> String {
        U::symbol()
    }

    #[inline]
    fn unit_name() -> String {
        U::unit_name()
    }
}
