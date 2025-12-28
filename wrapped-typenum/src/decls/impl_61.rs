macro_rules! deps {
    () => {
        Z0!();
        Unsigned!();
        NInt!();
        NonZero!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [doc = " `NInt + Z0 = NInt`"] impl < U : Unsigned + NonZero > Add < Z0 > for NInt < U > { type Output = NInt < U > ; # [inline] fn add (self , _ : Z0) -> Self :: Output { NInt :: new () } }
    };
}

impl_61!();