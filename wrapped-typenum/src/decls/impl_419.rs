macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
        UInt!();
        Bit!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        # [doc = " `UInt<U, B> * UTerm = UTerm`"] impl < U : Unsigned , B : Bit > Mul < UTerm > for UInt < U , B > { type Output = UTerm ; # [inline] fn mul (self , _ : UTerm) -> Self :: Output { UTerm } }
    };
}

impl_419!()