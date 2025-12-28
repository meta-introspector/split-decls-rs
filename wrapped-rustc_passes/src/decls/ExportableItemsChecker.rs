macro_rules! ExportableItemsChecker {
    () => {
        struct ExportableItemsChecker < 'tcx , 'a > { tcx : TyCtxt < 'tcx > , exportable_items : & 'a FxIndexSet < DefId > , item_id : DefId , }
    };
}

ExportableItemsChecker!()