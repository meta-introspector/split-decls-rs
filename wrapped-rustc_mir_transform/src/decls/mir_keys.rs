macro_rules! mir_keys {
    () => {
        # [doc = " Finds the full set of `DefId`s within the current crate that have"] # [doc = " MIR associated with them."] fn mir_keys (tcx : TyCtxt < '_ > , () : ()) -> FxIndexSet < LocalDefId > { let mut set : FxIndexSet < _ > = tcx . hir_body_owners () . collect () ; set . retain (| & def_id | ! matches ! (tcx . def_kind (def_id) , DefKind :: GlobalAsm)) ; for body_owner in tcx . hir_body_owners () { if let DefKind :: Closure = tcx . def_kind (body_owner) && tcx . needs_coroutine_by_move_body_def_id (body_owner . to_def_id ()) { set . insert (tcx . coroutine_by_move_body_def_id (body_owner) . expect_local ()) ; } } for item in tcx . hir_crate_items (()) . free_items () { if let DefKind :: Struct | DefKind :: Enum = tcx . def_kind (item . owner_id) { for variant in tcx . adt_def (item . owner_id) . variants () { if let Some ((CtorKind :: Fn , ctor_def_id)) = variant . ctor { set . insert (ctor_def_id . expect_local ()) ; } } } } set }
    };
}

mir_keys!()