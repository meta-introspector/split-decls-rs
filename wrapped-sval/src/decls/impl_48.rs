macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "failed to stream data") } }
    };
}

impl_48!()