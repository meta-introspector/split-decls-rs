macro_rules! impl_item_implementor_ids {
    () => {
        fn impl_item_implementor_ids (tcx : TyCtxt < '_ > , impl_id : DefId) -> DefIdMap < DefId > { tcx . associated_items (impl_id) . in_definition_order () . filter_map (| item | item . trait_item_def_id () . map (| trait_item | (trait_item , item . def_id))) . collect () }
    };
}

impl_item_implementor_ids!();