// Generated macro for collect_item (function)
macro_rules! Depcrate_diagnostic_itemscollect_item {
() => {
// Module: crate::diagnostic_items
// Provides: {"collect_item"}
// Dependencies: {}
fn collect_item (tcx : TyCtxt < '_ > , items : & mut DiagnosticItems , name : Symbol , item_def_id : DefId) { items . id_to_name . insert (item_def_id , name) ; if let Some (original_def_id) = items . name_to_id . insert (name , item_def_id) { if original_def_id != item_def_id { report_duplicate_item (tcx , name , original_def_id , item_def_id) ; } } }
};
}
