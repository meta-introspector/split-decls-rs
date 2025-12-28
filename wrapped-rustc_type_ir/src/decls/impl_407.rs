macro_rules! deps {
    () => {
        AliasRelationDirection!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl std :: fmt :: Display for AliasRelationDirection { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { AliasRelationDirection :: Equate => write ! (f , "==") , AliasRelationDirection :: Subtype => write ! (f , "<:") , } } }
    };
}

impl_407!();