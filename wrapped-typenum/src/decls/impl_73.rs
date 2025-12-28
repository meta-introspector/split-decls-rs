macro_rules! deps {
    () => {
        Z0!();
        NonZero!();
        NInt!();
        Unsigned!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        # [doc = " `NInt - Z0 = NInt`"] impl < U : Unsigned + NonZero > Sub < Z0 > for NInt < U > { type Output = NInt < U > ; # [inline] fn sub (self , _ : Z0) -> Self :: Output { NInt :: new () } }
    };
}

impl_73!()