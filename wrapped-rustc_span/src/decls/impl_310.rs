macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl fmt :: Debug for SourceFile { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "SourceFile({:?})" , self . name) } }
    };
}

impl_310!()