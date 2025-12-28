macro_rules! deps {
    () => {
        B0!();
        Unsigned!();
        UInt!();
        Bit!();
        Prod!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B0> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0>`"] impl < Ul : Unsigned , B : Bit , Ur : Unsigned > Mul < UInt < Ur , B > > for UInt < Ul , B0 > where Ul : Mul < UInt < Ur , B > > , { type Output = UInt < Prod < Ul , UInt < Ur , B > > , B0 > ; # [inline] fn mul (self , rhs : UInt < Ur , B >) -> Self :: Output { UInt { msb : self . msb * rhs , lsb : B0 , } } }
    };
}

impl_421!();