macro_rules! deps {
    () => {
        PathError!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Display for PathError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} at path {:?}" , self . err , self . path) } }
    };
}

impl_19!();