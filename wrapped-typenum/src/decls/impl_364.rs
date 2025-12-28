macro_rules! deps {
    () => {
        B1!();
        B0!();
        UInt!();
        Unsigned!();
        Sum!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> + UInt<Ur, B1> = UInt<Ul + Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > Add < UInt < Ur , B1 > > for UInt < Ul , B0 > where Ul : Add < Ur > , { type Output = UInt < Sum < Ul , Ur > , B1 > ; # [inline] fn add (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb + rhs . msb , lsb : B1 , } } }
    };
}

impl_364!()