macro_rules! deps {
    () => {
        BuildMetadata!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Debug for BuildMetadata { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "BuildMetadata(\"{}\")" , self) } }
    };
}

impl_8!()