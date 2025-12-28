macro_rules! deps {
    () => {
        CompressionMethod!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl fmt :: Display for CompressionMethod { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:?}") } }
    };
}

impl_35!();