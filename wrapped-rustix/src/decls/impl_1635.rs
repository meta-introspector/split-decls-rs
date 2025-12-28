macro_rules! deps {
    () => {
        Result!();
        DecInt!();
    };
}

macro_rules! impl_1635 {
    () => {
        deps!();
        impl fmt :: Debug for DecInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_1635!()