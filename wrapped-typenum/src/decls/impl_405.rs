macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        # [doc = " Shifting left `UInt` by `UTerm`: `UInt<U, B> << UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Shl < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shl (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
    };
}

impl_405!()