macro_rules! deps {
    () => {
        Interner!();
        TypeError!();
        ExpectedFound!();
        AliasTerm!();
        Relate!();
        TypeRelation!();
        UnevaluatedConst!();
        AliasTermKind!();
        RelateResult!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < I : Interner > Relate < I > for ty :: AliasTerm < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: AliasTerm < I > , b : ty :: AliasTerm < I > ,) -> RelateResult < I , ty :: AliasTerm < I > > { if a . def_id != b . def_id { Err (TypeError :: ProjectionMismatched ({ let a = a . def_id ; let b = b . def_id ; ExpectedFound :: new (a , b) })) } else { let args = match a . kind (relation . cx ()) { ty :: AliasTermKind :: OpaqueTy => relate_args_with_variances (relation , a . def_id , relation . cx () . variances_of (a . def_id) , a . args , b . args , false ,) ? , ty :: AliasTermKind :: ProjectionTy | ty :: AliasTermKind :: FreeConst | ty :: AliasTermKind :: FreeTy | ty :: AliasTermKind :: InherentTy | ty :: AliasTermKind :: InherentConst | ty :: AliasTermKind :: UnevaluatedConst | ty :: AliasTermKind :: ProjectionConst => { relate_args_invariantly (relation , a . args , b . args) ? } } ; Ok (ty :: AliasTerm :: new_from_args (relation . cx () , a . def_id , args)) } } }
    };
}

impl_119!();