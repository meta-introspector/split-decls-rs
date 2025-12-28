macro_rules! deps {
    () => {
        DeserializerError!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Display for DeserializerError { # [inline] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { DeserializerError (msg) => write ! (f , "{}" , msg) , } } }
    };
}

impl_3!();