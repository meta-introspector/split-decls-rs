macro_rules! deps {
    () => {
        PrivateXorOut!();
        Unsigned!();
        PrivateXor!();
        UInt!();
        B0!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> ^ UInt<Ur, B0> = UInt<Ul ^ Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateXor < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : PrivateXor < Ur > , { type Output = UInt < PrivateXorOut < Ul , Ur > , B0 > ; # [inline] fn private_xor (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . private_xor (rhs . msb) , lsb : B0 , } } }
    };
}

impl_397!();