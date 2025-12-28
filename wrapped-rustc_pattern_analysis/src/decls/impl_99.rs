macro_rules! deps {
    () => {
        PlaceValidity!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl fmt :: Display for PlaceValidity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match self { ValidOnly => "✓" , MaybeInvalid => "?" , } ; write ! (f , "{s}") } }
    };
}

impl_99!();