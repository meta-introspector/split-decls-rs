macro_rules! deps {
    () => {
        Unsigned!();
        PrivateSub!();
        UInt!();
        B0!();
        B1!();
        PrivateSubOut!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B1> - UInt<Ur, B1> = UInt<Ul - Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateSub < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : PrivateSub < Ur > , { type Output = UInt < PrivateSubOut < Ul , Ur > , B0 > ; # [inline] fn private_sub (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . private_sub (rhs . msb) , lsb : B0 , } } }
    };
}

impl_378!();