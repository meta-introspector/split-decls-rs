macro_rules! deps {
    () => {
        FileAstId!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < N > fmt :: Debug for FileAstId < N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "FileAstId::<{}>({:?})" , type_name ::< N > () , self . raw) } }
    };
}

impl_20!();