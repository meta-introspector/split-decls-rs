macro_rules! deps {
    () => {
        Sum!();
        B0!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> + UInt<Ur, B0> = UInt<Ul + Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > Add < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : Add < Ur > , { type Output = UInt < Sum < Ul , Ur > , B0 > ; # [inline] fn add (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb + rhs . msb , lsb : B0 , } } }
    };
}

impl_363!()