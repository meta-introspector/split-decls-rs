macro_rules! deps {
    () => {
        TraitPredicate!();
        Interner!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < I : Interner > fmt :: Debug for TraitPredicate < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "TraitPredicate({:?}, polarity:{:?})" , self . trait_ref , self . polarity) } }
    };
}

impl_353!()