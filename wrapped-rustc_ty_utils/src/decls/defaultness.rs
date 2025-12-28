macro_rules! defaultness {
    () => {
        fn defaultness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> hir :: Defaultness { match tcx . hir_node_by_def_id (def_id) { hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Impl (hir :: Impl { of_trait : Some (hir :: TraitImplHeader { defaultness , .. }) , .. }) , .. }) | hir :: Node :: ImplItem (hir :: ImplItem { impl_kind : hir :: ImplItemImplKind :: Trait { defaultness , .. } , .. }) | hir :: Node :: TraitItem (hir :: TraitItem { defaultness , .. }) => * defaultness , node => { bug ! ("`defaultness` called on {:?}" , node) ; } } }
    };
}

defaultness!()