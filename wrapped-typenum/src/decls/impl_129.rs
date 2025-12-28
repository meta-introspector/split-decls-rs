macro_rules! deps {
    () => {
        Min!();
        Unsigned!();
        NInt!();
        NonZero!();
        Z0!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < U > Min < NInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = NInt < U > ; # [inline] fn min (self , rhs : NInt < U >) -> Self :: Output { rhs } }
    };
}

impl_129!();