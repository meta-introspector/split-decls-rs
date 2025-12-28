macro_rules! deps {
    () => {
        Bit!();
        B0!();
        UInt!();
        UTerm!();
        Unsigned!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        # [doc = " `UInt * B0 = UTerm`"] impl < U : Unsigned , B : Bit > Mul < B0 > for UInt < U , B > { type Output = UTerm ; # [inline] fn mul (self , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_415!()