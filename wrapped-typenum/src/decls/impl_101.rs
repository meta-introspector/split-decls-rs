macro_rules! deps {
    () => {
        Z0!();
        Integer!();
        NonZero!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        # [doc = " `Z0 % I = Z0` where `I != 0`"] impl < I : Integer + NonZero > Rem < I > for Z0 { type Output = Z0 ; # [inline] fn rem (self , _ : I) -> Self :: Output { Z0 } }
    };
}

impl_101!()