macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl fmt :: Display for dyn Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self , f) } }
    };
}

impl_151!();