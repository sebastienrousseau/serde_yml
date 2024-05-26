/// Macro `from_number` to implement conversion from number types to `Value`.
#[macro_export]
macro_rules! from_number {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Number(n.into())
                }
            }
        )*
    };
}
