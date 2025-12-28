macro_rules! deps {
    () => {
        Lifetime!();
        Result!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl Display for Lifetime { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { "'" . fmt (formatter) ? ; self . ident . fmt (formatter) } }
    };
}

impl_399!();