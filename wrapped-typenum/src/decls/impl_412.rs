macro_rules! deps {
    () => {
        Unsigned!();
        UInt!();
        B0!();
        Bit!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        # [doc = " Shifting right any unsigned by a zero bit: `U >> B0 = U`"] impl < U : Unsigned , B : Bit > Shr < B0 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shr (self , _ : B0) -> Self :: Output { UInt :: new () } }
    };
}

impl_412!()