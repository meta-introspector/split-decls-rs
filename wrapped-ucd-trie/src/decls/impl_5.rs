macro_rules! deps {
    () => {
        TrieSetSlice!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for TrieSetSlice < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "TrieSetSlice(...)") } }
    };
}

impl_5!()