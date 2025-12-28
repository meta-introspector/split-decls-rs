macro_rules! deps {
    () => {
        PrivateAnd!();
        UInt!();
        Bit!();
        UTerm!();
        Unsigned!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        # [doc = " `X & UTerm = UTerm`"] impl < B : Bit , U : Unsigned > PrivateAnd < UTerm > for UInt < U , B > { type Output = UTerm ; # [inline] fn private_and (self , _ : UTerm) -> Self :: Output { UTerm } }
    };
}

impl_382!();