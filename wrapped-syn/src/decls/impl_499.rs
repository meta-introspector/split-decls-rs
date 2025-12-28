macro_rules! deps {
    () => {
        Result!();
        ParseBuffer!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl < 'a > Display for ParseBuffer < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . cursor () . token_stream () , f) } }
    };
}

impl_499!()