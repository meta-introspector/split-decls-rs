macro_rules! deps {
    () => {
        UnevaluatedConst!();
        Interner!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < I : Interner > Eq for UnevaluatedConst < I > { }
    };
}

impl_280!()