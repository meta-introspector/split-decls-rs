macro_rules! deps {
    () => {
        Unaligned!();
        Unalign!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl < T : Unaligned + Debug > Debug for Unalign < T > { # [inline (always)] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Debug :: fmt (self . deref () , f) } }
    };
}

impl_420!();