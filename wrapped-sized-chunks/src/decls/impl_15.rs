macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < A , T > Default for InlineArray < A , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_15!()