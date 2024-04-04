use alloc::format;
use alloc::string::{String, ToString};

use crate::{
    ops::{Div, Mul, Pow},
    units::Unit,
};

/// A unit with an assosicated name and symbol
pub trait NamedUnit: Unit {
    const NEEDS_GROUPING: bool = false;

    fn unit_name() -> String;
    fn symbol() -> String;
}

impl<L: NamedUnit, R: NamedUnit> NamedUnit for Mul<L, R> {
    fn symbol() -> String {
        format!("{}•{}", L::symbol(), R::symbol())
    }

    fn unit_name() -> String {
        let left_unit = match L::NEEDS_GROUPING {
            true => format!("({})", L::unit_name()),
            false => L::unit_name(),
        };

        format!("{} {}", left_unit, R::unit_name())
    }
}

impl<L: NamedUnit, R: NamedUnit> NamedUnit for Div<L, R> {
    const NEEDS_GROUPING: bool = true;

    fn symbol() -> String {
        format!("{}•{}⁻¹", L::symbol(), R::symbol())
    }

    fn unit_name() -> String {
        let left_unit = match L::NEEDS_GROUPING {
            true => format!("({})", L::unit_name()),
            false => L::unit_name(),
        };

        format!("{} per {}", left_unit, R::unit_name())
    }
}

impl<U: NamedUnit, const P: i32> NamedUnit for Pow<U, P> {
    fn unit_name() -> String {
        format!("({}){}", U::unit_name(), num_to_superscript(P))
    }

    fn symbol() -> String {
        format!("({}){}", U::symbol(), num_to_superscript(P))
    }
}

/// Convert a number to superscript (for unit powers)
fn num_to_superscript<N: ToString>(n: N) -> String {
    n.to_string()
        .chars()
        .filter_map(|c| match c {
            '1' => Some('¹'),
            '2' => Some('²'),
            '3' => Some('³'),
            '4' => Some('⁴'),
            '5' => Some('⁵'),
            '6' => Some('⁶'),
            '7' => Some('⁷'),
            '8' => Some('⁸'),
            '9' => Some('⁹'),
            '0' => Some('⁰'),
            '-' => Some('⁻'),
            _ => None,
        })
        .collect::<String>()
}
