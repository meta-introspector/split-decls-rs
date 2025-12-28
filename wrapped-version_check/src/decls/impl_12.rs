macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Display for Version { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (major , minor , patch) = self . to_mmp () ; write ! (f , "{}.{}.{}" , major , minor , patch) } }
    };
}

impl_12!();