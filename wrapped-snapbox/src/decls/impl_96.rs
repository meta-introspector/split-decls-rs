macro_rules! deps {
    () => {
        Position!();
        Result!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl std :: fmt :: Display for Position { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}:{}:{}" , crate :: dir :: display_relpath (& self . file) , self . line , self . column) } }
    };
}

impl_96!();