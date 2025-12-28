macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < A , T > Eq for InlineArray < A , T > where A : Eq { }
    };
}

impl_24!()