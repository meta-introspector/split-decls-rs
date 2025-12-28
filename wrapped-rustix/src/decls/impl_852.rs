macro_rules! deps {
    () => {
        DecInt!();
        Result!();
    };
}

macro_rules! impl_852 {
    () => {
        deps!();
        impl fmt :: Debug for DecInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_852!()