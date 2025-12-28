macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
        Bit!();
        UInt!();
    };
}

macro_rules! impl_409 {
    () => {
        deps!();
        # [doc = " Shifting right `UInt` by `UTerm`: `UInt<U, B> >> UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Shr < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shr (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
    };
}

impl_409!()