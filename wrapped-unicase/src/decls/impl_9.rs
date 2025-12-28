macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < S : fmt :: Display > fmt :: Display for Ascii < S > { # [inline] fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , fmt) } }
    };
}

impl_9!();