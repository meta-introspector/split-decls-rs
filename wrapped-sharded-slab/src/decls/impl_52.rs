macro_rules! deps {
    () => {
        DefaultConfig!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl fmt :: Debug for DefaultConfig { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Self :: debug () . fmt (f) } }
    };
}

impl_52!()