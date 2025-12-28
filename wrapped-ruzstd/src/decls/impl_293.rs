macro_rules! deps {
    () => {
        Sequence!();
        Error!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl core :: fmt :: Display for Sequence { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { write ! (f , "LL: {}, ML: {}, OF: {}" , self . ll , self . ml , self . of) } }
    };
}

impl_293!();