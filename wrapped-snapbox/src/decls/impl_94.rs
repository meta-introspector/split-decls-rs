macro_rules! deps {
    () => {
        Inline!();
        Result!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl std :: fmt :: Display for Inline { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . position . fmt (f) } }
    };
}

impl_94!()