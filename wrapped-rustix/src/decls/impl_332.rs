macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < T > :: core :: cmp :: Eq for UnionField < T > { }
    };
}

impl_332!();