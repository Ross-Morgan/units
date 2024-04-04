/// Create a new base unit
#[macro_export]
macro_rules! unit {
    ($name:ident, $type_name:expr, $symbol:expr) => {
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        struct $name;

        impl $crate::units::Unit for $name {
            type Mul<R: $crate::units::Unit> = $crate::ops::Mul<$name, R>;
            type Div<R: $crate::units::Unit> = $crate::ops::Div<$name, R>;
            type Pow<const P: i32> = $crate::ops::Pow<$name, P>;
        }

        impl $crate::name::NamedUnit for $name {
            fn unit_name() -> String {
                String::from($type_name)
            }

            fn symbol() -> String {
                String::from($symbol)
            }
        }
    };
}

/// Create a value with an associated unit
#[macro_export]
macro_rules! value {
    ($v:expr; $unit:ty) => {
        $crate::values::Value::<_, $unit>::new($v)
    };
}
