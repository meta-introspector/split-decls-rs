macro_rules! deps {
    () => {
        UInt!();
        B0!();
        B1!();
        Unsigned!();
        Bit!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        # [doc = " `UInt<U, B1> - B1 = UInt<U, B0>`"] impl < U : Unsigned , B : Bit > Sub < B1 > for UInt < UInt < U , B > , B1 > { type Output = UInt < UInt < U , B > , B0 > ; # [inline] fn sub (self , _ : B1) -> Self :: Output { UInt :: new () } }
    };
}

impl_369!();