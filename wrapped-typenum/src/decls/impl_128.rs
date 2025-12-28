macro_rules! deps {
    () => {
        PInt!();
        Unsigned!();
        NonZero!();
        Z0!();
        Min!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < U > Min < PInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = Z0 ; # [inline] fn min (self , _ : PInt < U >) -> Self :: Output { self } }
    };
}

impl_128!()