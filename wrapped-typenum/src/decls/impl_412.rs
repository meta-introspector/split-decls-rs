macro_rules! deps {
    () => {
        B0!();
        Bit!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        # [doc = " Shifting right any unsigned by a zero bit: `U >> B0 = U`"] impl < U : Unsigned , B : Bit > Shr < B0 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shr (self , _ : B0) -> Self :: Output { UInt :: new () } }
    };
}

impl_412!();