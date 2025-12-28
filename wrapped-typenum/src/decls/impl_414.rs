macro_rules! deps {
    () => {
        B1!();
        Sub1!();
        UInt!();
        Shright!();
        Unsigned!();
        Bit!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        # [doc = " Shifting right `UInt` by `UInt`: `UInt(U, B) >> Y` = `U >> (Y - 1)`"] impl < U : Unsigned , B : Bit , Ur : Unsigned , Br : Bit > Shr < UInt < Ur , Br > > for UInt < U , B > where UInt < Ur , Br > : Sub < B1 > , U : Shr < Sub1 < UInt < Ur , Br > > > , { type Output = Shright < U , Sub1 < UInt < Ur , Br > > > ; # [inline] fn shr (self , rhs : UInt < Ur , Br >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] self . msb . shr (rhs - B1) } }
    };
}

impl_414!()