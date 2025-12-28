macro_rules! deps {
    () => {
        ValueRepr!();
        Repr!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl ValueRepr for bool { fn to_repr (& self) -> Repr { let repr = self . to_toml_value () ; Repr :: new_unchecked (repr) } }
    };
}

impl_59!();