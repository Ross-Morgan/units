use crate::name::NamedUnit;
use crate::ops::{Div, Mul, Pow};
use crate::units::Unit;

use alloc::{format, string::String};

pub trait Multiplier: Unit {
    const POWER: i32;
    const SYMBOL: char;
    const NAME: &'static str;
}

macro_rules! multipliers {
    { $($multiplier_name:ident, $name:literal => $power:literal, $symbol:literal),+ $(,)? } => {
        $(
            pub struct $multiplier_name<U: Unit>(core::marker::PhantomData<U>);

            impl<L: Unit> Unit for $multiplier_name<L> {
                type Mul<R: Unit> = Mul<$multiplier_name<L>, R>;
                type Div<R: Unit> = Div<$multiplier_name<L>, R>;
                type Pow<const P: i32> = Pow<$multiplier_name<L>, P>;
            }

            impl<U: Unit> Multiplier for $multiplier_name<U> {
                const POWER: i32 = $power;
                const SYMBOL: char = $symbol;
                const NAME: &'static str = $name;
            }

            impl<U: NamedUnit> NamedUnit for $multiplier_name<U> {
                fn unit_name() -> String {
                    format!("{}{}", Self::NAME, U::unit_name().to_lowercase())
                }

                fn symbol() -> String {
                    format!("{}{}", Self::SYMBOL, U::symbol())
                }
            }
        )+
    };
}

#[rustfmt::skip]
multipliers! {
    Deca,   "Deca"   => 1,  'D',
    Hecto,  "Hecto"  => 2,  'h',
    Kilo,   "Kilo"   => 3,  'k',
    Mega,   "Mega"   => 6,  'M',
    Giga,   "Giga"   => 9,  'G',
    Tera,   "Tera"   => 12, 'T',
    Peta,   "Peta"   => 15, 'P',
    Exa,    "Exa"    => 18, 'E',
    Zetta,  "Zetta"  => 21, 'Z',
    Yotta,  "Yotta"  => 24, 'Y',
    Ronna,  "Ronna"  => 27, 'R',
    Quetta, "Quetta" => 30, 'Q',
}

#[rustfmt::skip]
multipliers! {
    Deci,   "Deci"   => -1,  'd',
    Centi,  "Centi"  => -2,  'c',
    Milli,  "Milli"  => -3,  'm',
    Micro,  "Micro"  => -6,  'μ',
    Nano,   "Nano"   => -9,  'n',
    Pico,   "Pico"   => -12, 'p',
    Femto,  "Femto"  => -15, 'f',
    Atto,   "Atto"   => -18, 'a',
    Zepto,  "Zepto"  => -21, 'z',
    Yocto,  "Yocto"  => -24, 'y',
    Ronto,  "Ronto"  => -27, 'r',
    Quecto, "Quecto" => -30, 'q',
}
