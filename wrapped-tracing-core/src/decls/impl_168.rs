macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl fmt :: Display for Field { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (self . name ()) } }
    };
}

impl_168!();