macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < S : fmt :: Display > fmt :: Display for UniCase < S > { # [inline] fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (inner ! (self . 0) , fmt) } }
    };
}

impl_18!()