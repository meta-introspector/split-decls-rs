macro_rules! deps {
    () => {
        TrieSetSlice!();
        Result!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for TrieSetSlice < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "TrieSetSlice(...)") } }
    };
}

impl_56!();