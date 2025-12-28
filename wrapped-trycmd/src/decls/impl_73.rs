macro_rules! deps {
    () => {
        Filesystem!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl std :: fmt :: Display for Filesystem { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { for status in & self . context { status . fmt (f) ? ; } Ok (()) } }
    };
}

impl_73!();