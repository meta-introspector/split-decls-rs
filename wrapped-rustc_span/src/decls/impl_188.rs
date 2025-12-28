macro_rules! deps {
    () => {
        MacroRulesNormalizedIdent!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl fmt :: Debug for MacroRulesNormalizedIdent { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_188!()