macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl fmt :: Display for Uuid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self , f) } }
    };
}

impl_45!()