macro_rules! deps {
    () => {
        Canonical!();
        Interner!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < I : Interner , V : fmt :: Display > fmt :: Display for Canonical < I , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { value , max_universe , variables } = self ; write ! (f , "Canonical {{ value: {value}, max_universe: {max_universe:?}, variables: {variables:?} }}" ,) } }
    };
}

impl_265!();