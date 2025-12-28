macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < A , T > Borrow < [A] > for InlineArray < A , T > { fn borrow (& self) -> & [A] { self . deref () } }
    };
}

impl_19!();