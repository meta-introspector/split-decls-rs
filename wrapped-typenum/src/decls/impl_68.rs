macro_rules! deps {
    () => {
        Less!();
        Unsigned!();
        NonZero!();
        NInt!();
        PrivateIntegerAdd!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [doc = " `P + N = Negative` where `P < N`"] impl < N : Unsigned , P : Unsigned > PrivateIntegerAdd < Less , N > for P where N : Sub < P > , < N as Sub < P > > :: Output : Unsigned + NonZero , { type Output = NInt < < N as Sub < P > > :: Output > ; # [inline] fn private_integer_add (self , _ : Less , n : N) -> Self :: Output { NInt { n : n - self } } }
    };
}

impl_68!();