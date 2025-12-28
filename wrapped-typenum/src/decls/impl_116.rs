macro_rules! deps {
    () => {
        Unsigned!();
        Pow!();
        UInt!();
        B0!();
        NInt!();
        PInt!();
        NonZero!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        # [doc = " N(Ul)^P(Ur) = P(Ul^Ur) if Ur is even"] impl < Ul : Unsigned + NonZero , Ur : Unsigned > Pow < PInt < UInt < Ur , B0 > > > for NInt < Ul > where Ul : Pow < UInt < Ur , B0 > > , < Ul as Pow < UInt < Ur , B0 > > > :: Output : Unsigned + NonZero , { type Output = PInt < < Ul as Pow < UInt < Ur , B0 > > > :: Output > ; # [inline] fn powi (self , _ : PInt < UInt < Ur , B0 > >) -> Self :: Output { PInt :: new () } }
    };
}

impl_116!()