macro_rules! deps {
    () => {
        NonNilUuid!();
        Uuid!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl fmt :: Display for NonNilUuid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& Uuid :: from (* self) , f) } }
    };
}

impl_23!()