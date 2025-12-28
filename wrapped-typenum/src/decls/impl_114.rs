macro_rules! deps {
    () => {
        Unsigned!();
        NInt!();
        Z0!();
        NonZero!();
        Pow!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        # [doc = " N^0 = 1"] impl < U : Unsigned + NonZero > Pow < Z0 > for NInt < U > { type Output = P1 ; # [inline] fn powi (self , _ : Z0) -> Self :: Output { P1 :: new () } }
    };
}

impl_114!()