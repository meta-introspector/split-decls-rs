macro_rules! deps {
    () => {
        TypeError!();
        ExpectedFound!();
        ExistentialTraitRef!();
        Interner!();
        TypeRelation!();
        Relate!();
        RelateResult!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < I : Interner > Relate < I > for ty :: ExistentialTraitRef < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: ExistentialTraitRef < I > , b : ty :: ExistentialTraitRef < I > ,) -> RelateResult < I , ty :: ExistentialTraitRef < I > > { if a . def_id != b . def_id { Err (TypeError :: Traits ({ let a = a . def_id ; let b = b . def_id ; ExpectedFound :: new (a , b) })) } else { let args = relate_args_invariantly (relation , a . args , b . args) ? ; Ok (ty :: ExistentialTraitRef :: new_from_args (relation . cx () , a . def_id , args)) } } }
    };
}

impl_122!();