macro_rules! deps {
    () => {
        Result!();
        ParseBuffer!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < 'a > Debug for ParseBuffer < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . cursor () . token_stream () , f) } }
    };
}

impl_500!();