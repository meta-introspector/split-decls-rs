macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl fmt :: UpperHex for Uuid { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self . as_hyphenated () , f) } }
    };
}

impl_49!()