macro_rules! deps {
    () => {
        GenericArgs!();
        DefId!();
        TypeRelation!();
        Interner!();
        VarianceDiagInfo!();
        RelateResult!();
    };
}

macro_rules! relate_args_with_variances {
    () => {
        deps!();
        pub fn relate_args_with_variances < I : Interner , R : TypeRelation < I > > (relation : & mut R , ty_def_id : I :: DefId , variances : I :: VariancesOf , a_arg : I :: GenericArgs , b_arg : I :: GenericArgs , fetch_ty_for_diag : bool ,) -> RelateResult < I , I :: GenericArgs > { let cx = relation . cx () ; let mut cached_ty = None ; let params = iter :: zip (a_arg . iter () , b_arg . iter ()) . enumerate () . map (| (i , (a , b)) | { let variance = variances . get (i) . unwrap () ; let variance_info = if variance == ty :: Invariant && fetch_ty_for_diag { let ty = * cached_ty . get_or_insert_with (| | cx . type_of (ty_def_id) . instantiate (cx , a_arg)) ; VarianceDiagInfo :: Invariant { ty , param_index : i . try_into () . unwrap () } } else { VarianceDiagInfo :: default () } ; relation . relate_with_variance (variance , variance_info , a , b) }) ; cx . mk_args_from_iter (params) }
    };
}

relate_args_with_variances!();