macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Uuid { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self , f) } }
    };
}

impl_44!();