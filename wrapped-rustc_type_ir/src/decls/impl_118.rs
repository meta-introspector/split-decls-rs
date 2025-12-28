macro_rules! deps {
    () => {
        TypeRelation!();
        Interner!();
        RelateResult!();
        Relate!();
        AliasTy!();
        TypeError!();
        ExpectedFound!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < I : Interner > Relate < I > for ty :: AliasTy < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: AliasTy < I > , b : ty :: AliasTy < I > ,) -> RelateResult < I , ty :: AliasTy < I > > { if a . def_id != b . def_id { Err (TypeError :: ProjectionMismatched ({ let a = a . def_id ; let b = b . def_id ; ExpectedFound :: new (a , b) })) } else { let cx = relation . cx () ; let args = if let Some (variances) = cx . opt_alias_variances (a . kind (cx) , a . def_id) { relate_args_with_variances (relation , a . def_id , variances , a . args , b . args , false ,) ? } else { relate_args_invariantly (relation , a . args , b . args) ? } ; Ok (ty :: AliasTy :: new_from_args (relation . cx () , a . def_id , args)) } } }
    };
}

impl_118!()