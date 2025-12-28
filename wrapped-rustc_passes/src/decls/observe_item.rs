macro_rules! observe_item {
    () => {
        fn observe_item < 'tcx > (tcx : TyCtxt < 'tcx > , diagnostic_items : & mut DiagnosticItems , owner : OwnerId) { let attrs = tcx . hir_attrs (owner . into ()) ; if let Some (name) = extract (attrs) { collect_item (tcx , diagnostic_items , name , owner . to_def_id ()) ; } }
    };
}

observe_item!();