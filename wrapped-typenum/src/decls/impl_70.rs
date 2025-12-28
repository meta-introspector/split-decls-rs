macro_rules! deps {
    () => {
        Z0!();
        Unsigned!();
        PInt!();
        NonZero!();
        NInt!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        # [doc = " `Z0 - P = N`"] impl < U : Unsigned + NonZero > Sub < PInt < U > > for Z0 { type Output = NInt < U > ; # [inline] fn sub (self , _ : PInt < U >) -> Self :: Output { NInt :: new () } }
    };
}

impl_70!();