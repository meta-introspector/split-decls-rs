macro_rules! deps {
    () => {
        UInt!();
        Bit!();
        Prod!();
        Sum!();
        B0!();
        B1!();
        Unsigned!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        # [doc = " `UInt<Ul, B1> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0> + UInt<Ur, B>`"] impl < Ul : Unsigned , B : Bit , Ur : Unsigned > Mul < UInt < Ur , B > > for UInt < Ul , B1 > where Ul : Mul < UInt < Ur , B > > , UInt < Prod < Ul , UInt < Ur , B > > , B0 > : Add < UInt < Ur , B > > , { type Output = Sum < UInt < Prod < Ul , UInt < Ur , B > > , B0 > , UInt < Ur , B > > ; # [inline] fn mul (self , rhs : UInt < Ur , B >) -> Self :: Output { UInt { msb : self . msb * rhs , lsb : B0 , } + rhs } }
    };
}

impl_422!();