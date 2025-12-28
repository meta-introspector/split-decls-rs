macro_rules! deps {
    () => {
        SequencesHeaderParseError!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl core :: fmt :: Display for SequencesHeaderParseError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { SequencesHeaderParseError :: NotEnoughBytes { need_at_least , got } => { write ! (f , "source must have at least {need_at_least} bytes to parse header; got {got} bytes" ,) } } } }
    };
}

impl_99!();