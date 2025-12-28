macro_rules! deps {
    () => {
        ProjectionPredicate!();
        Interner!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < I : Interner > fmt :: Debug for ProjectionPredicate < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ProjectionPredicate({:?}, {:?})" , self . projection_term , self . term) } }
    };
}

impl_385!()