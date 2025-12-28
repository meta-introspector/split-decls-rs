macro_rules! deps {
    () => {
        Relate!();
        TypeRelation!();
        ExpectedFound!();
        TraitRef!();
        TypeError!();
        Interner!();
        RelateResult!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < I : Interner > Relate < I > for ty :: TraitRef < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: TraitRef < I > , b : ty :: TraitRef < I > ,) -> RelateResult < I , ty :: TraitRef < I > > { if a . def_id != b . def_id { Err (TypeError :: Traits ({ let a = a . def_id ; let b = b . def_id ; ExpectedFound :: new (a , b) })) } else { let args = relate_args_invariantly (relation , a . args , b . args) ? ; Ok (ty :: TraitRef :: new_from_args (relation . cx () , a . def_id , args)) } } }
    };
}

impl_121!()