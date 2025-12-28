macro_rules! deps {
    () => {
        B0!();
        Unsigned!();
        UInt!();
        B1!();
        Bit!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        # [doc = " Shifting left a `UInt` by a one bit: `UInt<U, B> << B1 = UInt<UInt<U, B>, B0>`"] impl < U : Unsigned , B : Bit > Shl < B1 > for UInt < U , B > { type Output = UInt < UInt < U , B > , B0 > ; # [inline] fn shl (self , _ : B1) -> Self :: Output { UInt :: new () } }
    };
}

impl_404!();