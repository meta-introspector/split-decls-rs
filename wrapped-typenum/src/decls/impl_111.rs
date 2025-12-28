macro_rules! deps {
    () => {
        Unsigned!();
        B0!();
        Pow!();
        UInt!();
        NInt!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        # [doc = " (-1)^N = 1 if N is even"] impl < U : Unsigned > Pow < NInt < UInt < U , B0 > > > for N1 { type Output = P1 ; # [inline] fn powi (self , _ : NInt < UInt < U , B0 > >) -> Self :: Output { P1 :: new () } }
    };
}

impl_111!();