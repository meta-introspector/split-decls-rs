macro_rules! deps {
    () => {
        B0!();
        Unsigned!();
        PrivateSub!();
        PrivateSubOut!();
        UInt!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> - UInt<Ur, B0> = UInt<Ul - Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateSub < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : PrivateSub < Ur > , { type Output = UInt < PrivateSubOut < Ul , Ur > , B0 > ; # [inline] fn private_sub (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . private_sub (rhs . msb) , lsb : B0 , } } }
    };
}

impl_375!();