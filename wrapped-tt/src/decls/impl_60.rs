macro_rules! deps {
    () => {
        Punct!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < S > fmt :: Display for Punct < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . char , f) } }
    };
}

impl_60!();