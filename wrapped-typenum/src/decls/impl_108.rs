macro_rules! deps {
    () => {
        Pow!();
        Unsigned!();
        PInt!();
        NonZero!();
        Z0!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        # [doc = " 0^P = 0"] impl < U : Unsigned + NonZero > Pow < PInt < U > > for Z0 { type Output = Z0 ; # [inline] fn powi (self , _ : PInt < U >) -> Self :: Output { Z0 } }
    };
}

impl_108!();