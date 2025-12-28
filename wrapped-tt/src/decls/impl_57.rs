macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < S > fmt :: Display for Ident < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . is_raw . as_str () , f) ? ; fmt :: Display :: fmt (& self . sym , f) } }
    };
}

impl_57!();