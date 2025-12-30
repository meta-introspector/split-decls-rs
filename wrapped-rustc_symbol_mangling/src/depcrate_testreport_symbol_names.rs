// Generated macro for report_symbol_names (function)
macro_rules! Depcrate_testreport_symbol_names {
() => {
// Module: crate::test
// Provides: {"report_symbol_names"}
// Dependencies: {}
pub fn report_symbol_names (tcx : TyCtxt < '_ >) { if ! tcx . features () . rustc_attrs () { return ; } tcx . dep_graph . with_ignore (| | { let mut symbol_names = SymbolNamesTest { tcx } ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . free_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . trait_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . impl_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . foreign_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } }) }
};
}
