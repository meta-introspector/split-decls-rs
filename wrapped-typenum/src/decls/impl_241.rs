macro_rules! deps {
    () => {
        InvertedUnsigned!();
        PrivateInvert!();
        UTerm!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < IU : InvertedUnsigned > PrivateInvert < IU > for UTerm { type Output = IU ; # [inline] fn private_invert (self , rhs : IU) -> Self :: Output { rhs } }
    };
}

impl_241!();