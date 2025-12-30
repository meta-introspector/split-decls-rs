// Generated macro for observe_item (function)
macro_rules! Depcrate_diagnostic_itemsobserve_item {
() => {
// Module: crate::diagnostic_items
// Provides: {"observe_item"}
// Dependencies: {}
fn observe_item < 'tcx > (tcx : TyCtxt < 'tcx > , diagnostic_items : & mut DiagnosticItems , owner : OwnerId) { let attrs = tcx . hir_attrs (owner . into ()) ; if let Some (name) = extract (attrs) { collect_item (tcx , diagnostic_items , name , owner . to_def_id ()) ; } }
};
}
