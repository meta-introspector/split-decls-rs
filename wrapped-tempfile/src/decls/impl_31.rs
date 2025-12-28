macro_rules! deps {
    () => {
        PathPersistError!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl fmt :: Display for PathPersistError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to persist temporary file path: {}" , self . error) } }
    };
}

impl_31!();