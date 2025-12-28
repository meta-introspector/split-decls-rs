macro_rules! deps {
    () => {
        B0!();
        Unsigned!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        # [doc = " Shifting left any unsigned by a zero bit: `U << B0 = U`"] impl < U : Unsigned , B : Bit > Shl < B0 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shl (self , _ : B0) -> Self :: Output { UInt :: new () } }
    };
}

impl_403!();