macro_rules! deps {
    () => {
        Z0!();
        Unsigned!();
        NonZero!();
        Min!();
        NInt!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < U > Min < NInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = NInt < U > ; # [inline] fn min (self , rhs : NInt < U >) -> Self :: Output { rhs } }
    };
}

impl_129!()