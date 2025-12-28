macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < T > :: core :: marker :: Copy for UnionField < T > { }
    };
}

impl_328!()