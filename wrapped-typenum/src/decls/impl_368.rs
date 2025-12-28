macro_rules! deps {
    () => {
        UInt!();
        Bit!();
        Unsigned!();
        B0!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        # [doc = " `UInt - B0 = UInt`"] impl < U : Unsigned , B : Bit > Sub < B0 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn sub (self , _ : B0) -> Self :: Output { UInt :: new () } }
    };
}

impl_368!()