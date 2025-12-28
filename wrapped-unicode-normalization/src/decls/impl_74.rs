macro_rules! deps {
    () => {
        Recompositions!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > + Clone > fmt :: Display for Recompositions < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for c in self . clone () { f . write_char (c) ? ; } Ok (()) } }
    };
}

impl_74!();