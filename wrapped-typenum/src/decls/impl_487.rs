macro_rules! deps {
    () => {
        Unsigned!();
        Min!();
        UTerm!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < U > Min < U > for UTerm where U : Unsigned , { type Output = UTerm ; # [inline] fn min (self , _ : U) -> Self :: Output { self } }
    };
}

impl_487!()