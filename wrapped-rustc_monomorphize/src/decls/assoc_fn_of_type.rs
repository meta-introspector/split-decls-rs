macro_rules! assoc_fn_of_type {
    () => {
        fn assoc_fn_of_type < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , fn_ident : Ident) -> Option < DefId > { for impl_def_id in tcx . inherent_impls (def_id) { if let Some (new) = tcx . associated_items (impl_def_id) . find_by_ident_and_kind (tcx , fn_ident , AssocTag :: Fn , def_id ,) { return Some (new . def_id) ; } } None }
    };
}

assoc_fn_of_type!();