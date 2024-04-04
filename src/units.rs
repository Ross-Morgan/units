/// A unit that can be combined with other units
pub trait Unit {
    type Mul<R: Unit>: Unit;
    type Div<R: Unit>: Unit;
    type Pow<const P: i32>: Unit;
}
