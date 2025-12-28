macro_rules! deps {
    () => {
        B0!();
        UInt!();
        B1!();
        PrivateXorOut!();
        PrivateXor!();
        Unsigned!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B1> ^ UInt<Ur, B0> = UInt<Ul ^ Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateXor < UInt < Ur , B0 > > for UInt < Ul , B1 > where Ul : PrivateXor < Ur > , { type Output = UInt < PrivateXorOut < Ul , Ur > , B1 > ; # [inline] fn private_xor (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . private_xor (rhs . msb) , lsb : B1 , } } }
    };
}

impl_399!()