use core::marker::PhantomData;

use crate::units::Unit;

pub struct Mul<L: Unit, R: Unit>(PhantomData<(L, R)>);
pub struct Div<L: Unit, R: Unit>(PhantomData<(L, R)>);
pub struct Pow<U: Unit, const P: i32>(PhantomData<U>);

impl<L: Unit, R: Unit> Unit for Mul<L, R> {
    type Mul<R2: Unit> = Mul<Mul<L, R>, R2>;
    type Div<R2: Unit> = Div<Mul<L, R>, R2>;
    type Pow<const P: i32> = Pow<Mul<L, R>, P>;
}

impl<L: Unit, R: Unit> Unit for Div<L, R> {
    type Mul<R2: Unit> = Mul<Div<L, R>, R2>;
    type Div<R2: Unit> = Div<Div<L, R>, R2>;
    type Pow<const P: i32> = Pow<Div<L, R>, P>;
}

impl<L: Unit, const P: i32> Unit for Pow<L, P> {
    type Mul<R: Unit> = Mul<Pow<L, P>, R>;
    type Div<R: Unit> = Div<Pow<L, P>, R>;
    type Pow<const P2: i32> = Pow<Pow<L, P>, P2>;
}