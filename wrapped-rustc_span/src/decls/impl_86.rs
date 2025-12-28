macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl fmt :: Debug for SourceFile { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "SourceFile({:?})" , self . name) } }
    };
}

impl_86!()