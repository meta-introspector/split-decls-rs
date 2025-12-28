macro_rules! deps {
    () => {
        Interner!();
        NormalizesTo!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < I : Interner > fmt :: Debug for NormalizesTo < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "NormalizesTo({:?}, {:?})" , self . alias , self . term) } }
    };
}

impl_389!()