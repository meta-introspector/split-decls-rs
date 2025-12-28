macro_rules! deps {
    () => {
        B0!();
        Unsigned!();
        PrivateXor!();
        UInt!();
        B1!();
        PrivateXorOut!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> ^ UInt<Ur, B1> = UInt<Ul ^ Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateXor < UInt < Ur , B1 > > for UInt < Ul , B0 > where Ul : PrivateXor < Ur > , { type Output = UInt < PrivateXorOut < Ul , Ur > , B1 > ; # [inline] fn private_xor (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . private_xor (rhs . msb) , lsb : B1 , } } }
    };
}

impl_398!();