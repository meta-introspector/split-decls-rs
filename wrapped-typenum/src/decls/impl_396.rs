macro_rules! deps {
    () => {
        UInt!();
        PrivateXor!();
        Unsigned!();
        UTerm!();
        Bit!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        # [doc = " `X ^ UTerm = X`"] impl < B : Bit , U : Unsigned > PrivateXor < UTerm > for UInt < U , B > { type Output = Self ; # [inline] fn private_xor (self , _ : UTerm) -> Self :: Output { self } }
    };
}

impl_396!()