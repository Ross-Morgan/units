use core::fmt::Display;
use core::marker::PhantomData;
use core::ops as arith;

use alloc::format;
use alloc::string::{String, ToString};

use crate::name::NamedUnit;
use crate::ops::{Div, Mul};
use crate::units::Unit;

/// A struct containing a value and a
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value<T, U: Unit>(T, PhantomData<U>);

impl<T, U: Unit> Value<T, U> {
    pub const fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: Copy, U: Unit> Value<T, U> {
    pub const fn into_innner(&self) -> T {
        self.0
    }
}

impl<T, U: Unit> arith::Add<Value<T, U>> for Value<T, U>
where
    T: arith::Add<T, Output = T>,
{
    type Output = Value<T, U>;

    fn add(self, rhs: Value<T, U>) -> Self::Output {
        Self::Output::new(self.0 + rhs.0)
    }
}

impl<T, U: Unit> arith::Sub<Value<T, U>> for Value<T, U>
where
    T: arith::Sub<T, Output = T>,
{
    type Output = Value<T, U>;

    fn sub(self, rhs: Value<T, U>) -> Self::Output {
        Self::Output::new(self.0 - rhs.0)
    }
}

impl<T, U1: Unit, U2: Unit> arith::Mul<Value<T, U2>> for Value<T, U1>
where
    T: arith::Mul<T, Output = T>,
{
    type Output = Value<T, Mul<U1, U2>>;

    fn mul(self, rhs: Value<T, U2>) -> Self::Output {
        Self::Output::new(self.0 * rhs.0)
    }
}

impl<T, U1: Unit, U2: Unit> arith::Div<Value<T, U2>> for Value<T, U1>
where
    T: arith::Div<T, Output = T>,
{
    type Output = Value<T, Div<U1, U2>>;

    fn div(self, rhs: Value<T, U2>) -> Self::Output {
        Self::Output::new(self.0 / rhs.0)
    }
}

impl<T: Copy + Display, U: NamedUnit> ToString for Value<T, U> {
    fn to_string(&self) -> String {
        format!("{} {}", self.0, U::symbol())
    }
}
