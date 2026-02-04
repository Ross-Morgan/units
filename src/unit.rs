use core::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use alloc::string::ToString;
use typenum::{Integer, Z0};

type AddInts<A, B> = <A as Add<B>>::Output;
type SubInts<A, B> = <A as Sub<B>>::Output;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SI<
    V,
    L: Integer = Z0,
    M: Integer = Z0,
    T: Integer = Z0,
    I: Integer = Z0,
    O: Integer = Z0,
    N: Integer = Z0,
    J: Integer = Z0,
>(V, core::marker::PhantomData<(L, M, T, I, O, N, J)>);

impl<V, L1, M1, T1, I1, O1, N1, J1, L2, M2, T2, I2, O2, N2, J2>
    Mul<SI<V, L2, M2, T2, I2, O2, N2, J2>> for SI<V, L1, M1, T1, I1, O1, N1, J1>
where
    V: Mul<Output = V>,
    L1: Integer + Add<L2>,
    M1: Integer + Add<M2>,
    T1: Integer + Add<T2>,
    I1: Integer + Add<I2>,
    O1: Integer + Add<O2>,
    N1: Integer + Add<N2>,
    J1: Integer + Add<J2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    I2: Integer,
    O2: Integer,
    N2: Integer,
    J2: Integer,
    AddInts<L1, L2>: Integer,
    AddInts<M1, M2>: Integer,
    AddInts<T1, T2>: Integer,
    AddInts<I1, I2>: Integer,
    AddInts<O1, O2>: Integer,
    AddInts<N1, N2>: Integer,
    AddInts<J1, J2>: Integer,
{
    type Output = SI<
        V,
        AddInts<L1, L2>,
        AddInts<M1, M2>,
        AddInts<T1, T2>,
        AddInts<I1, I2>,
        AddInts<O1, O2>,
        AddInts<N1, N2>,
        AddInts<J1, J2>,
    >;

    fn mul(self, rhs: SI<V, L2, M2, T2, I2, O2, N2, J2>) -> Self::Output {
        Self::Output::new(self.0 * rhs.0)
    }
}

impl<V, L1, M1, T1, I1, O1, N1, J1, L2, M2, T2, I2, O2, N2, J2>
    Div<SI<V, L2, M2, T2, I2, O2, N2, J2>> for SI<V, L1, M1, T1, I1, O1, N1, J1>
where
    V: Div<Output = V>,
    L1: Integer + Sub<L2>,
    M1: Integer + Sub<M2>,
    T1: Integer + Sub<T2>,
    I1: Integer + Sub<I2>,
    O1: Integer + Sub<O2>,
    N1: Integer + Sub<N2>,
    J1: Integer + Sub<J2>,
    L2: Integer,
    M2: Integer,
    T2: Integer,
    I2: Integer,
    O2: Integer,
    N2: Integer,
    J2: Integer,
    SubInts<L1, L2>: Integer,
    SubInts<M1, M2>: Integer,
    SubInts<T1, T2>: Integer,
    SubInts<I1, I2>: Integer,
    SubInts<O1, O2>: Integer,
    SubInts<N1, N2>: Integer,
    SubInts<J1, J2>: Integer,
{
    type Output = SI<
        V,
        SubInts<L1, L2>,
        SubInts<M1, M2>,
        SubInts<T1, T2>,
        SubInts<I1, I2>,
        SubInts<O1, O2>,
        SubInts<N1, N2>,
        SubInts<J1, J2>,
    >;

    fn div(self, rhs: SI<V, L2, M2, T2, I2, O2, N2, J2>) -> Self::Output {
        Self::Output::new(self.0 / rhs.0)
    }
}

impl<V, L, M, T, I, O, N, J> Add for SI<V, L, M, T, I, O, N, J>
where
    V: Add<Output = V>,
    L: Integer,
    M: Integer,
    T: Integer,
    I: Integer,
    O: Integer,
    N: Integer,
    J: Integer,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.0 + rhs.0)
    }
}

impl<V, L, M, T, I, O, N, J> Sub for SI<V, L, M, T, I, O, N, J>
where
    V: Sub<Output = V>,
    L: Integer,
    M: Integer,
    T: Integer,
    I: Integer,
    O: Integer,
    N: Integer,
    J: Integer,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.0 - rhs.0)
    }
}

impl<V, L, M, T, I, O, N, J> SI<V, L, M, T, I, O, N, J>
where
    L: Integer,
    M: Integer,
    T: Integer,
    I: Integer,
    O: Integer,
    N: Integer,
    J: Integer,
{
    pub const fn new(value: V) -> Self {
        Self(value, core::marker::PhantomData)
    }

    pub const fn dimensions(&self) -> [i8; 7] {
        [L::I8, M::I8, T::I8, I::I8, O::I8, N::I8, J::I8]
    }

    pub const fn dimension(&self, d: Dimension) -> i8 {
        match d {
            Dimension::Length => L::I8,
            Dimension::Mass => M::I8,
            Dimension::Time => T::I8,
            Dimension::Current => I::I8,
            Dimension::Temperature => O::I8,
            Dimension::Amount => N::I8,
            Dimension::LuminousIntensity => J::I8,
        }
    }
}

pub enum Dimension {
    Length,
    Mass,
    Time,
    Current,
    Temperature,
    Amount,
    LuminousIntensity,
}

impl<V, L, M, T, I, O, N, J> From<V> for SI<V, L, M, T, I, O, N, J>
where
    L: Integer,
    M: Integer,
    T: Integer,
    I: Integer,
    O: Integer,
    N: Integer,
    J: Integer,
{
    fn from(value: V) -> Self {
        Self::new(value)
    }
}

impl<V, L, M, T, I, O, N, J> ToString for SI<V, L, M, T, I, O, N, J>
where
    V: ToString,
    L: Integer,
    M: Integer,
    T: Integer,
    I: Integer,
    O: Integer,
    N: Integer,
    J: Integer,
{
    fn to_string(&self) -> alloc::string::String {
        let mut out = alloc::string::String::new();

        let v = self.0;

        let l = L::I8;
        let 


    }
}

fn to_superscript(n: i64) -> String {
    map_chars(&n.to_string(), |c| match c {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '-' => '⁻',
        '+' => '⁺',
        _ => c,
    })
}