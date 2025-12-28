macro_rules! deps {
    () => {
        Liveness!();
        IrMaps!();
    };
}

macro_rules! check_liveness {
    () => {
        deps!();
        fn check_liveness (tcx : TyCtxt < '_ > , def_id : LocalDefId) { let parent = tcx . local_parent (def_id) ; if let DefKind :: Impl { .. } = tcx . def_kind (parent) && find_attr ! (tcx . get_all_attrs (parent) , AttributeKind :: AutomaticallyDerived (..)) { return ; } if find_attr ! (tcx . get_all_attrs (def_id . to_def_id ()) , AttributeKind :: Naked (..)) { return ; } let mut maps = IrMaps :: new (tcx) ; let body = tcx . hir_body_owned_by (def_id) ; let hir_id = tcx . hir_body_owner (body . id ()) ; if let Some (upvars) = tcx . upvars_mentioned (def_id) { for & var_hir_id in upvars . keys () { let var_name = tcx . hir_name (var_hir_id) ; maps . add_variable (Upvar (var_hir_id , var_name)) ; } } maps . visit_body (& body) ; let mut lsets = Liveness :: new (& mut maps , def_id) ; let entry_ln = lsets . compute (& body , hir_id) ; lsets . log_liveness (entry_ln , body . id () . hir_id) ; lsets . visit_body (& body) ; lsets . warn_about_unused_upvars (entry_ln) ; lsets . warn_about_unused_args (& body , entry_ln) ; }
    };
}

check_liveness!();