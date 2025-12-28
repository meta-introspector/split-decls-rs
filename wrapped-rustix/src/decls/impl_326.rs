macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl < T > :: core :: default :: Default for UnionField < T > { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_326!()