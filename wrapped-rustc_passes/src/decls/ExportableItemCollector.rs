macro_rules! ExportableItemCollector {
    () => {
        struct ExportableItemCollector < 'tcx > { tcx : TyCtxt < 'tcx > , exportable_items : FxIndexSet < DefId > , in_exportable_mod : bool , seen_exportable_in_mod : bool , }
    };
}

ExportableItemCollector!();