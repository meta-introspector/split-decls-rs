macro_rules! associated_item {
    () => {
        fn associated_item (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: AssocItem { let assoc_item = match tcx . hir_node_by_def_id (def_id) { hir :: Node :: TraitItem (ti) => associated_item_from_trait_item (tcx , ti) , hir :: Node :: ImplItem (ii) => associated_item_from_impl_item (tcx , ii) , node => span_bug ! (tcx . def_span (def_id) , "impl item or item not found: {:?}" , node ,) , } ; debug_assert_eq ! (assoc_item . def_id . expect_local () , def_id) ; assoc_item }
    };
}

associated_item!();