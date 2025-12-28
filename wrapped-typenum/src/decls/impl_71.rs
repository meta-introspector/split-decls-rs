macro_rules! deps {
    () => {
        Unsigned!();
        PInt!();
        NonZero!();
        NInt!();
        Z0!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [doc = " `Z0 - N = P`"] impl < U : Unsigned + NonZero > Sub < NInt < U > > for Z0 { type Output = PInt < U > ; # [inline] fn sub (self , _ : NInt < U >) -> Self :: Output { PInt :: new () } }
    };
}

impl_71!();