macro_rules! deps {
    () => {
        DefId!();
        Interner!();
        GenericArgs!();
        UnevaluatedConst!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl < I : Interner > UnevaluatedConst < I > { # [inline] pub fn new (def : I :: DefId , args : I :: GenericArgs) -> UnevaluatedConst < I > { UnevaluatedConst { def , args } } }
    };
}

impl_281!()