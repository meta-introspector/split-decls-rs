macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < A , T > AsRef < [A] > for InlineArray < A , T > { fn as_ref (& self) -> & [A] { self . deref () } }
    };
}

impl_21!()