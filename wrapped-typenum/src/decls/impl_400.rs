macro_rules! deps {
    () => {
        PrivateXor!();
        B1!();
        Unsigned!();
        B0!();
        UInt!();
        PrivateXorOut!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B1> ^ UInt<Ur, B1> = UInt<Ul ^ Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateXor < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : PrivateXor < Ur > , { type Output = UInt < PrivateXorOut < Ul , Ur > , B0 > ; # [inline] fn private_xor (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . private_xor (rhs . msb) , lsb : B0 , } } }
    };
}

impl_400!();