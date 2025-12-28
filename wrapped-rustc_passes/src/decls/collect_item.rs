macro_rules! collect_item {
    () => {
        fn collect_item (tcx : TyCtxt < '_ > , items : & mut DiagnosticItems , name : Symbol , item_def_id : DefId) { items . id_to_name . insert (item_def_id , name) ; if let Some (original_def_id) = items . name_to_id . insert (name , item_def_id) { if original_def_id != item_def_id { report_duplicate_item (tcx , name , original_def_id , item_def_id) ; } } }
    };
}

collect_item!();