macro_rules! deps {
    () => {
        PersistError!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < F > fmt :: Display for PersistError < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to persist temporary file: {}" , self . error) } }
    };
}

impl_47!()