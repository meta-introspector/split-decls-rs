macro_rules! deps {
    () => {
        Decompositions!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > + Clone > fmt :: Display for Decompositions < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for c in self . clone () { f . write_char (c) ? ; } Ok (()) } }
    };
}

impl_9!()