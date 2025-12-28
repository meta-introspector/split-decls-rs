macro_rules! deps {
    () => {
        UTerm!();
        B0!();
        PrivateSetBit!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl < I > PrivateSetBit < I , B0 > for UTerm { type Output = UTerm ; # [inline] fn private_set_bit (self , _ : I , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_466!();