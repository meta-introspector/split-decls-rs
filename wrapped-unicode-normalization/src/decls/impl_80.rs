macro_rules! deps {
    () => {
        Replacements!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > + Clone > fmt :: Display for Replacements < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for c in self . clone () { f . write_char (c) ? ; } Ok (()) } }
    };
}

impl_80!();