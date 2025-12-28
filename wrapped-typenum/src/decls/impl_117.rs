macro_rules! deps {
    () => {
        NonZero!();
        B1!();
        Unsigned!();
        PInt!();
        NInt!();
        UInt!();
        Pow!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        # [doc = " N(Ul)^P(Ur) = N(Ul^Ur) if Ur is odd"] impl < Ul : Unsigned + NonZero , Ur : Unsigned > Pow < PInt < UInt < Ur , B1 > > > for NInt < Ul > where Ul : Pow < UInt < Ur , B1 > > , < Ul as Pow < UInt < Ur , B1 > > > :: Output : Unsigned + NonZero , { type Output = NInt < < Ul as Pow < UInt < Ur , B1 > > > :: Output > ; # [inline] fn powi (self , _ : PInt < UInt < Ur , B1 > >) -> Self :: Output { NInt :: new () } }
    };
}

impl_117!()