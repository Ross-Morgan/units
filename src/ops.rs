use std::marker::PhantomData;

use crate::units::Unit;

pub struct Mul<T: Copy, L: Unit<T>, R: Unit<T>>(T, PhantomData<(L, R)>);
pub struct Div<T: Copy, L: Unit<T>, R: Unit<T>>(T, PhantomData<(L, R)>);

impl<T: Copy, L: Unit<T>, R: Unit<T>> Unit<T> for Mul<T, L, R> {
    type Mul<R2: Unit<T>> = Mul<T, Mul<T, L, R>, R2>;
    type Div<R2: Unit<T>> = Div<T, Mul<T, L, R>, R2>;

    fn value(&self) -> T {
        self.0
    }
}

impl<T: Copy, L: Unit<T>, R: Unit<T>> Unit<T> for Div<T, L, R> {
    type Mul<R2: Unit<T>> = Mul<T, Div<T, L, R>, R2>;
    type Div<R2: Unit<T>> = Div<T, Div<T, L, R>, R2>;

    fn value(&self) -> T {
        self.0
    }
}
