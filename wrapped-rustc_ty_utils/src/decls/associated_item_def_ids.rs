macro_rules! associated_item_def_ids {
    () => {
        fn associated_item_def_ids (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> & [DefId] { let item = tcx . hir_expect_item (def_id) ; match item . kind { hir :: ItemKind :: Trait (.. , trait_item_refs) => { let rpitit_items = tcx . associated_types_for_impl_traits_in_trait_or_impl (def_id) ; tcx . arena . alloc_from_iter (trait_item_refs . iter () . flat_map (| trait_item_ref | { let item_def_id = trait_item_ref . owner_id . to_def_id () ; [item_def_id] . into_iter () . chain (rpitit_items . get (& item_def_id) . into_iter () . flatten () . copied ()) })) } hir :: ItemKind :: Impl (impl_) => { let rpitit_items = tcx . associated_types_for_impl_traits_in_trait_or_impl (def_id) ; tcx . arena . alloc_from_iter (impl_ . items . iter () . flat_map (| impl_item_ref | { let item_def_id = impl_item_ref . owner_id . to_def_id () ; [item_def_id] . into_iter () . chain (rpitit_items . get (& item_def_id) . into_iter () . flatten () . copied ()) })) } _ => span_bug ! (item . span , "associated_item_def_ids: not impl or trait") , } }
    };
}

associated_item_def_ids!();