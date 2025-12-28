macro_rules! deps {
    () => {
        Prerelease!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Debug for Prerelease { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "Prerelease(\"{}\")" , self) } }
    };
}

impl_7!()