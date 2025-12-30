// Generated macro for report_duplicate_item (function)
macro_rules! Depcrate_diagnostic_itemsreport_duplicate_item {
() => {
// Module: crate::diagnostic_items
// Provides: {"report_duplicate_item"}
// Dependencies: {}
fn report_duplicate_item (tcx : TyCtxt < '_ > , name : Symbol , original_def_id : DefId , item_def_id : DefId ,) { let orig_span = tcx . hir_span_if_local (original_def_id) ; let duplicate_span = tcx . hir_span_if_local (item_def_id) ; tcx . dcx () . emit_err (DuplicateDiagnosticItemInCrate { duplicate_span , orig_span , crate_name : tcx . crate_name (item_def_id . krate) , orig_crate_name : tcx . crate_name (original_def_id . krate) , different_crates : (item_def_id . krate != original_def_id . krate) , name , }) ; }
};
}
