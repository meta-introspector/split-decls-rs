macro_rules! deps {
    () => {
        TrieSetOwned!();
        Result!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl fmt :: Debug for TrieSetOwned { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "TrieSetOwned(...)") } }
    };
}

impl_47!()