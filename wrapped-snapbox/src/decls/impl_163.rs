macro_rules! deps {
    () => {
        Result!();
        FileType!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl std :: fmt :: Display for FileType { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_163!();