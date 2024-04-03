#[macro_export]
macro_rules! unit {
    ($name:ident ($ty:ty), $type_name:expr, $symbol:expr) => {
        $($v)? struct $name($ty);

        impl $crate::units::Unit for $name {
            type Mul<R: $crate::units::Unit> = $crate::ops::Mul<$name, R>;
            type Div<R: $crate::units::Unit> = $crate::ops::Div<$name, R>;
        }

        impl $crate::name::NamedUnit for $name {
            fn type_name() -> String {
                String::from($type_name)
            }

            fn symbol() -> String {
                String::from($symbol)
            }
        }

        impl $name {
            pub const fn inner(&self) -> $ty {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! unit_generic {
    ($name:ident, $type_name:expr, $symbol:expr) => {
        struct $name<T>(T);

        impl<T: Copy> $crate::units::Unit<T> for $name<T> {
            type Mul<R: $crate::units::Unit<T>> = $crate::ops::Mul<T, $name<T>, R>;
            type Div<R: $crate::units::Unit<T>> = $crate::ops::Div<T, $name<T>, R>;

            fn value(&self) -> T { self.0 }
        }

        impl<T: Copy> $crate::name::NamedUnit<T> for $name<T> {
            fn unit_name() -> String {
                String::from($type_name)
            }

            fn symbol() -> String {
                String::from($symbol)
            }
        }

        impl<T: Copy> $name<T> {
            pub const fn inner(&self) -> T {
                self.0
            }
        }

        impl<T> core::ops::Add<$name<T>> for $name<T>
        where T: core::ops::Add<T, Output = T>
        {
            type Output = $name<T>;

            fn add(self, rhs: $name<T>) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl<T> core::ops::Sub<$name<T>> for $name<T>
        where T: core::ops::Sub<T, Output = T>
        {
            type Output = $name<T>;

            fn sub(self, rhs: $name<T>) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl<T: Copy, R: $crate::units::Unit<T>> core::ops::Mul<R> for $name<T>
        where T: core::ops::Mul<T, Output = T>
        {
            type Output = $crate::compound::CompoundUnit<T, $crate::ops::Mul<T, $name<T>, R>>;

            fn mul(self, rhs: R) -> Self::Output {
                Self::Output::new($crate::units::Unit::value(&self) * rhs.value())
            }
        }

        impl<T: Copy, R: $crate::units::Unit<T>> core::ops::Div<R> for $name<T>
        where T: core::ops::Div<T, Output = T>
        {
            type Output = $crate::compound::CompoundUnit<T, $crate::ops::Div<T, $name<T>, R>>;

            fn div(self, rhs: R) -> Self::Output {
                Self::Output::new($crate::units::Unit::value(&self) / rhs.value())
            }
        }
    };
}
