macro_rules! deps {
    () => {
        Interner!();
        UnevaluatedConst!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < I : Interner > Eq for UnevaluatedConst < I > { }
    };
}

impl_280!();