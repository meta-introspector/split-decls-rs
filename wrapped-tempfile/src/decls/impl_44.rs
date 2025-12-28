macro_rules! deps {
    () => {
        PersistError!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < F > fmt :: Debug for PersistError < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "PersistError({:?})" , self . error) } }
    };
}

impl_44!()