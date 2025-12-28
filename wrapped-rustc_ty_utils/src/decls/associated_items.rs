macro_rules! associated_items {
    () => {
        fn associated_items (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: AssocItems { if tcx . is_trait_alias (def_id) { ty :: AssocItems :: new (Vec :: new ()) } else { let items = tcx . associated_item_def_ids (def_id) . iter () . map (| did | tcx . associated_item (* did)) ; ty :: AssocItems :: new (items) } }
    };
}

associated_items!()