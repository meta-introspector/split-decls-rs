macro_rules! deps {
    () => {
        B0!();
        Sub1!();
        UInt!();
        Unsigned!();
        B1!();
        Bit!();
        Shleft!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        # [doc = " Shifting left `UInt` by `UInt`: `X << Y` = `UInt(X, B0) << (Y - 1)`"] impl < U : Unsigned , B : Bit , Ur : Unsigned , Br : Bit > Shl < UInt < Ur , Br > > for UInt < U , B > where UInt < Ur , Br > : Sub < B1 > , UInt < UInt < U , B > , B0 > : Shl < Sub1 < UInt < Ur , Br > > > , { type Output = Shleft < UInt < UInt < U , B > , B0 > , Sub1 < UInt < Ur , Br > > > ; # [inline] fn shl (self , rhs : UInt < Ur , Br >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] (UInt { msb : self , lsb : B0 }) . shl (rhs - B1) } }
    };
}

impl_407!();