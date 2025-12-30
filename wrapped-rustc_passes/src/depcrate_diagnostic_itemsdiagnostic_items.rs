// Generated macro for diagnostic_items (function)
macro_rules! Depcrate_diagnostic_itemsdiagnostic_items {
() => {
// Module: crate::diagnostic_items
// Provides: {"diagnostic_items"}
// Dependencies: {}
# [doc = " Traverse and collect the diagnostic items in the current"] fn diagnostic_items (tcx : TyCtxt < '_ > , _ : LocalCrate) -> DiagnosticItems { let mut diagnostic_items = DiagnosticItems :: default () ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . owners () . chain (std :: iter :: once (CRATE_OWNER_ID)) { observe_item (tcx , & mut diagnostic_items , id) ; } diagnostic_items }
};
}
