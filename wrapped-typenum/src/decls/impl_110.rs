macro_rules! deps {
    () => {
        Pow!();
        Unsigned!();
        NonZero!();
        NInt!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        # [doc = " 1^N = 1"] impl < U : Unsigned + NonZero > Pow < NInt < U > > for P1 { type Output = P1 ; # [inline] fn powi (self , _ : NInt < U >) -> Self :: Output { P1 :: new () } }
    };
}

impl_110!();