macro_rules! deps {
    () => {
        NonZero!();
        PrivateIntegerAdd!();
        PInt!();
        Unsigned!();
        Greater!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        # [doc = " `P + N = Positive` where `P > N`"] impl < N : Unsigned , P : Unsigned > PrivateIntegerAdd < Greater , N > for P where P : Sub < N > , < P as Sub < N > > :: Output : Unsigned + NonZero , { type Output = PInt < < P as Sub < N > > :: Output > ; # [inline] fn private_integer_add (self , _ : Greater , n : N) -> Self :: Output { PInt { n : self - n } } }
    };
}

impl_67!();