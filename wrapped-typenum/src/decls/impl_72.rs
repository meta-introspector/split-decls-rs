macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        Z0!();
        PInt!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        # [doc = " `PInt - Z0 = PInt`"] impl < U : Unsigned + NonZero > Sub < Z0 > for PInt < U > { type Output = PInt < U > ; # [inline] fn sub (self , _ : Z0) -> Self :: Output { PInt :: new () } }
    };
}

impl_72!()