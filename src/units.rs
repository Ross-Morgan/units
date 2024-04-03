pub trait Unit<T> {
    type Mul<R: Unit<T>>: Unit<T>;
    type Div<R: Unit<T>>: Unit<T>;

    fn value(&self) -> T;
}
