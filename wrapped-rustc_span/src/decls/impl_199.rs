macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl fmt :: Display for Symbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_199!();