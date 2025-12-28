macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl fmt :: LowerHex for Uuid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self . as_hyphenated () , f) } }
    };
}

impl_48!();