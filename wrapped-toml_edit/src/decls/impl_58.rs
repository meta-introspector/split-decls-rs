macro_rules! deps {
    () => {
        Repr!();
        ValueRepr!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl ValueRepr for f64 { fn to_repr (& self) -> Repr { let repr = self . to_toml_value () ; Repr :: new_unchecked (repr) } }
    };
}

impl_58!();