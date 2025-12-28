macro_rules! deps {
    () => {
        B0!();
        Unsigned!();
        UInt!();
        B1!();
        Sum!();
        Add1!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B1> + UInt<Ur, B1> = UInt<(Ul + Ur) + B1, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > Add < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : Add < Ur > , Sum < Ul , Ur > : Add < B1 > , { type Output = UInt < Add1 < Sum < Ul , Ur > > , B0 > ; # [inline] fn add (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb + rhs . msb + B1 , lsb : B0 , } } }
    };
}

impl_366!();