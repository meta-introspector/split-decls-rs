macro_rules! deps {
    () => {
        Interner!();
        RelateResult!();
        TraitPredicate!();
        TypeRelation!();
        TypeError!();
        Relate!();
        ExpectedFound!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < I : Interner > Relate < I > for ty :: TraitPredicate < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: TraitPredicate < I > , b : ty :: TraitPredicate < I > ,) -> RelateResult < I , ty :: TraitPredicate < I > > { let trait_ref = relation . relate (a . trait_ref , b . trait_ref) ? ; if a . polarity != b . polarity { return Err (TypeError :: PolarityMismatch (ExpectedFound :: new (a . polarity , b . polarity))) ; } Ok (ty :: TraitPredicate { trait_ref , polarity : a . polarity }) } }
    };
}

impl_126!()