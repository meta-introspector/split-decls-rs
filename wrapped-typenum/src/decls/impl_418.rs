macro_rules! deps {
    () => {
        Bit!();
        B1!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        # [doc = " `UInt * B1 = UInt`"] impl < U : Unsigned , B : Bit > Mul < B1 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn mul (self , _ : B1) -> Self :: Output { UInt :: new () } }
    };
}

impl_418!()