macro_rules! deps {
    () => {
        Z0!();
        Max!();
        Unsigned!();
        PInt!();
        NonZero!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < U > Max < PInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = PInt < U > ; # [inline] fn max (self , rhs : PInt < U >) -> Self :: Output { rhs } }
    };
}

impl_137!()