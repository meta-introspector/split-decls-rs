macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < F > fmt :: Debug for NamedTempFile < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "NamedTempFile({:?})" , self . path) } }
    };
}

impl_41!();