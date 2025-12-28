macro_rules! deps {
    () => {
        NeedsDropTypes!();
        NeedsDropResult!();
        DtorType!();
    };
}

macro_rules! drop_tys_helper {
    () => {
        deps!();
        fn drop_tys_helper < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , adt_has_dtor : impl Fn (ty :: AdtDef < 'tcx >) -> Option < DtorType > , only_significant : bool , exhaustive : bool ,) -> impl Iterator < Item = NeedsDropResult < Ty < 'tcx > > > { fn with_query_cache < 'tcx > (tcx : TyCtxt < 'tcx > , iter : impl IntoIterator < Item = Ty < 'tcx > > ,) -> NeedsDropResult < Vec < Ty < 'tcx > > > { iter . into_iter () . try_fold (Vec :: new () , | mut vec , subty | { match subty . kind () { ty :: Adt (adt_id , args) => { for subty in tcx . adt_drop_tys (adt_id . did ()) ? { vec . push (EarlyBinder :: bind (subty) . instantiate (tcx , args)) ; } } _ => vec . push (subty) , } ; Ok (vec) }) } let adt_components = move | adt_def : ty :: AdtDef < 'tcx > , args : GenericArgsRef < 'tcx > | { if adt_def . is_manually_drop () { debug ! ("drop_tys_helper: `{:?}` is manually drop" , adt_def) ; Ok (Vec :: new ()) } else if let Some (dtor_info) = adt_has_dtor (adt_def) { match dtor_info { DtorType :: Significant => { debug ! ("drop_tys_helper: `{:?}` implements `Drop`" , adt_def) ; Err (AlwaysRequiresDrop) } DtorType :: Insignificant => { debug ! ("drop_tys_helper: `{:?}` drop is insignificant" , adt_def) ; Ok (args . types () . collect ()) } } } else if adt_def . is_union () { debug ! ("drop_tys_helper: `{:?}` is a union" , adt_def) ; Ok (Vec :: new ()) } else { let field_tys = adt_def . all_fields () . map (| field | { let r = tcx . type_of (field . did) . instantiate (tcx , args) ; debug ! ("drop_tys_helper: Instantiate into {:?} with {:?} getting {:?}" , field , args , r) ; r }) ; if only_significant { Ok (field_tys . collect ()) } else { with_query_cache (tcx , field_tys) } } . map (| v | v . into_iter ()) } ; NeedsDropTypes :: new (tcx , typing_env , ty , exhaustive , adt_components) }
    };
}

drop_tys_helper!();