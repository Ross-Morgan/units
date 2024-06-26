use alloc::string::String;

use crate::{
    ops::{Div, Mul},
    units::Unit,
};

pub trait NamedUnit: Unit {
    fn unit_name() -> String;
    fn symbol() -> String;
}

impl<L: Unit + NamedUnit, R: Unit + NamedUnit> NamedUnit for Mul<L, R> {
    fn symbol() -> String {
        alloc::format!("{}•{}", L::symbol(), R::symbol())
    }

    fn unit_name() -> String {
        alloc::format!("{}•{}", L::unit_name(), R::symbol())
    }
}

impl<L: Unit + NamedUnit, R: Unit + NamedUnit> NamedUnit for Div<L, R> {
    fn symbol() -> String {
        alloc::format!("{}•{}⁻¹", L::symbol(), R::symbol())
    }

    fn unit_name() -> String {
        alloc::format!("{}•{}⁻¹", L::unit_name(), R::symbol())
    }
}
