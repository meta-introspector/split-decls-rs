macro_rules! deps {
    () => {
        NonZero!();
        NInt!();
        Pow!();
        Z0!();
        Unsigned!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        # [doc = " 0^N = 0"] impl < U : Unsigned + NonZero > Pow < NInt < U > > for Z0 { type Output = Z0 ; # [inline] fn powi (self , _ : NInt < U >) -> Self :: Output { Z0 } }
    };
}

impl_109!()