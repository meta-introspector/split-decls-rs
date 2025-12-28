macro_rules! deps {
    () => {
        MacroRulesNormalizedIdent!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl fmt :: Display for MacroRulesNormalizedIdent { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_189!()