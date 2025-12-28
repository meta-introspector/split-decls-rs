macro_rules! inherit_const_stability {
    () => {
        fn inherit_const_stability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: AssocFn | DefKind :: AssocTy | DefKind :: AssocConst => { match tcx . def_kind (tcx . local_parent (def_id)) { DefKind :: Impl { of_trait : true } => true , _ => false , } } _ => false , } }
    };
}

inherit_const_stability!();