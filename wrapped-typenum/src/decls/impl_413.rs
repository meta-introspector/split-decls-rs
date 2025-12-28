macro_rules! deps {
    () => {
        Bit!();
        Unsigned!();
        B1!();
        UInt!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        # [doc = " Shifting right a `UInt` by a 1 bit: `UInt<U, B> >> B1 = U`"] impl < U : Unsigned , B : Bit > Shr < B1 > for UInt < U , B > { type Output = U ; # [inline] fn shr (self , _ : B1) -> Self :: Output { self . msb } }
    };
}

impl_413!();