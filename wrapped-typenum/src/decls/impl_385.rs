macro_rules! deps {
    () => {
        B0!();
        B1!();
        PrivateAndOut!();
        PrivateAnd!();
        Unsigned!();
        UInt!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B1> & UInt<Ur, B0> = UInt<Ul & Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateAnd < UInt < Ur , B0 > > for UInt < Ul , B1 > where Ul : PrivateAnd < Ur > , { type Output = UInt < PrivateAndOut < Ul , Ur > , B0 > ; # [inline] fn private_and (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . private_and (rhs . msb) , lsb : B0 , } } }
    };
}

impl_385!();