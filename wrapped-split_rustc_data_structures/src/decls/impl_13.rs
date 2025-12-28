macro_rules! deps {
    () => {
        BaseNString!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Display for BaseNString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self) } }
    };
}

impl_13!();