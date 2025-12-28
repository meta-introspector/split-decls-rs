macro_rules! deps {
    () => {
        Result!();
        DataError!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl std :: fmt :: Display for DataError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . error . fmt (f) } }
    };
}

impl_139!()