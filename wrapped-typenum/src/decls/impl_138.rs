macro_rules! deps {
    () => {
        NInt!();
        Z0!();
        Unsigned!();
        Max!();
        NonZero!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < U > Max < NInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = Z0 ; # [inline] fn max (self , _ : NInt < U >) -> Self :: Output { self } }
    };
}

impl_138!();