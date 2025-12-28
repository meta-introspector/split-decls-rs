macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < A , T > AsMut < [A] > for InlineArray < A , T > { fn as_mut (& mut self) -> & mut [A] { self . deref_mut () } }
    };
}

impl_22!();