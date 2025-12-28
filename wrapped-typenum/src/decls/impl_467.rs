macro_rules! deps {
    () => {
        PrivateSetBit!();
        Shleft!();
        B1!();
        UTerm!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < I > PrivateSetBit < I , B1 > for UTerm where U1 : Shl < I > , { type Output = Shleft < U1 , I > ; # [inline] fn private_set_bit (self , i : I , _ : B1) -> Self :: Output { < U1 as Shl < I > > :: shl (U1 :: new () , i) } }
    };
}

impl_467!()