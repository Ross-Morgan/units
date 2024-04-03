use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use crate::name::NamedUnit;
use crate::units::Unit;
use crate::ops::{Mul as UnitMul, Div as UnitDiv};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompoundUnit<T, U: Unit<T>>(pub(crate) T, pub(crate) PhantomData<U>);

impl<T, U: Unit<T>> CompoundUnit<T, U>
{
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: Copy, U: Unit<T>> Unit<T> for CompoundUnit<T, U> {
    type Mul<R: Unit<T>> = UnitMul<T, CompoundUnit<T, U>, R>;
    type Div<R: Unit<T>> = UnitDiv<T, CompoundUnit<T, U>, R>;

    fn value(&self) -> T {
        self.0
    }
}

impl<T, U: Unit<T>> Add<CompoundUnit<T, U>> for CompoundUnit<T, U>
where T: Add<T, Output = T>
{
    type Output = CompoundUnit<T, U>;

    fn add(self, rhs: CompoundUnit<T, U>) -> Self::Output {
        Self(self.0 + rhs.0, PhantomData)
    }
}

impl<T, U: Unit<T>> Sub<CompoundUnit<T, U>> for CompoundUnit<T, U>
where T: Sub<T, Output = T>
{
    type Output = CompoundUnit<T, U>;

    fn sub(self, rhs: CompoundUnit<T, U>) -> Self::Output {
        Self(self.0 - rhs.0, PhantomData)
    }
}

impl<T: Copy, LU: Unit<T>, RU: Unit<T>> Mul<CompoundUnit<T, RU>> for CompoundUnit<T, LU>
where T: Mul<T, Output = T>
{
    type Output = CompoundUnit<T, UnitMul<T, LU, RU>>;

    fn mul(self, rhs: CompoundUnit<T, RU>) -> Self::Output {
        CompoundUnit(self.0 * rhs.0, PhantomData)
    }
}

impl<T: Copy, LU: Unit<T>, RU: Unit<T>> Div<CompoundUnit<T, RU>> for CompoundUnit<T, LU>
where T: Div<T, Output = T>
{
    type Output = CompoundUnit<T, UnitDiv<T, LU, RU>>;

    fn div(self, rhs: CompoundUnit<T, RU>) -> Self::Output {
        CompoundUnit(self.0 / rhs.0, PhantomData)
    }
}

impl<T: Copy + Display, U: NamedUnit<T>> ToString for CompoundUnit<T, U> {
    fn to_string(&self) -> String {
        format!("{} {}", self.value(), Self::symbol())
    }
}
