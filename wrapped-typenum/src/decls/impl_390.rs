macro_rules! deps {
    () => {
        Or!();
        Unsigned!();
        B0!();
        B1!();
        UInt!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > BitOr < UInt < Ur , B1 > > for UInt < Ul , B0 > where Ul : BitOr < Ur > , { type Output = UInt < Or < Ul , Ur > , B1 > ; # [inline] fn bitor (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . bitor (rhs . msb) , lsb : self . lsb . bitor (rhs . lsb) , } } }
    };
}

impl_390!()