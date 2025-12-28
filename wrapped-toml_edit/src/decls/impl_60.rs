macro_rules! deps {
    () => {
        ValueRepr!();
        Repr!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl ValueRepr for Datetime { fn to_repr (& self) -> Repr { Repr :: new_unchecked (self . to_string ()) } }
    };
}

impl_60!();