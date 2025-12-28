macro_rules! deps {
    () => {
        PInt!();
        NonZero!();
        Z0!();
        Unsigned!();
        Pow!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        # [doc = " P^0 = 1"] impl < U : Unsigned + NonZero > Pow < Z0 > for PInt < U > { type Output = P1 ; # [inline] fn powi (self , _ : Z0) -> Self :: Output { P1 :: new () } }
    };
}

impl_113!();