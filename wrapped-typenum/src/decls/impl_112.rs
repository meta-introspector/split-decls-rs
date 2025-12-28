macro_rules! deps {
    () => {
        Pow!();
        B1!();
        Unsigned!();
        NInt!();
        UInt!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        # [doc = " (-1)^N = -1 if N is odd"] impl < U : Unsigned > Pow < NInt < UInt < U , B1 > > > for N1 { type Output = N1 ; # [inline] fn powi (self , _ : NInt < UInt < U , B1 > >) -> Self :: Output { N1 :: new () } }
    };
}

impl_112!();