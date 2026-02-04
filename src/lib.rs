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
    use crate::base::*;
    use core::ops::{Div, Mul};

    pub type Veloctiy<V> = <Metre<V> as Div<Second<V>>>::Output;
    pub type Acceleration<V> = <Veloctiy<V> as Div<Second<V>>>::Output;

    use typenum::{N2, P1, Z0};

    pub type Newton<V> = crate::SI<V, P1, P1, N2, Z0, Z0, Z0, Z0>;
}

pub use unit::{Dimension, SI};

pub use base::{Ampere, Candela, Kelvin, Kilogram, Metre, Mole, Second};

pub fn a() {
    let mass: Kilogram<i32> = base::Kilogram::new(10);
    let acceleration = derived::Acceleration::new(5);

    let force_a = mass * acceleration;
    let force_b = derived::Newton::new(30);
    let force_c = force_b - force_a;
}
