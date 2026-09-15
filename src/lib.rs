// #![feature(generic_const_exprs)]
// #![allow(incomplete_features)]
#![no_std]
#![warn(clippy::pedantic)]

extern crate alloc;

mod unit;
mod base {
    use crate::SI;
    use typenum::{P1, Z0};

    pub type Metre<V> = SI<V, P1, Z0, Z0, Z0, Z0, Z0, Z0>;
    pub type Kilogram<V> = SI<V, Z0, P1, Z0, Z0, Z0, Z0, Z0>;
    pub type Second<V> = SI<V, Z0, Z0, P1, Z0, Z0, Z0, Z0>;
    pub type Ampere<V> = SI<V, Z0, Z0, Z0, P1, Z0, Z0, Z0>;
    pub type Kelvin<V> = SI<V, Z0, Z0, Z0, Z0, P1, Z0, Z0>;
    pub type Mole<V> = SI<V, Z0, Z0, Z0, Z0, Z0, P1, Z0>;
    pub type Candela<V> = SI<V, Z0, Z0, Z0, Z0, Z0, Z0, P1>;
}

mod derived {
    use crate::base::{Metre, Second};
    use core::ops::Div;
    use typenum::{N2, P1, Z0};

    pub type Velocity<V> = <Metre<V> as Div<Second<V>>>::Output;
    pub type Acceleration<V> = <Velocity<V> as Div<Second<V>>>::Output;

    pub type Newton<V> = crate::SI<V, P1, P1, N2, Z0, Z0, Z0, Z0>;
}

pub use unit::{Dimension, SI};

pub use base::{Ampere, Candela, Kelvin, Kilogram, Metre, Mole, Second};
pub use derived::{Acceleration, Newton, Velocity};

/// Tests `F = m * a`
///
/// # Panics
///
/// If units fail to resolve, or if values are unequal, this will panic.
#[test]
pub fn unit_resolution() {
    let mass: Kilogram<i32> = base::Kilogram::new(10);
    let acceleration = derived::Acceleration::new(3);

    let force_a = mass * acceleration;
    let force_b = derived::Newton::new(30);

    assert_eq!(force_a, force_b);
}

pub fn same_type<T, const N: usize>(_: [T; N]) {}
