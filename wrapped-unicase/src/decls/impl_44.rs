macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < S : fmt :: Debug > fmt :: Debug for UniCase < S > { # [inline] fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (inner ! (self . 0) , fmt) } }
    };
}

impl_44!();