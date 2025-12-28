macro_rules! deps {
    () => {
        Unaligned!();
        Unalign!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl < T : Unaligned + Display > Display for Unalign < T > { # [inline (always)] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (self . deref () , f) } }
    };
}

impl_421!();