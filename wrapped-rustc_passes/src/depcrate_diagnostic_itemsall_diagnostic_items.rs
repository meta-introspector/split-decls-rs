// Generated macro for all_diagnostic_items (function)
macro_rules! Depcrate_diagnostic_itemsall_diagnostic_items {
() => {
// Module: crate::diagnostic_items
// Provides: {"all_diagnostic_items"}
// Dependencies: {}
# [doc = " Traverse and collect all the diagnostic items in all crates."] fn all_diagnostic_items (tcx : TyCtxt < '_ > , () : ()) -> DiagnosticItems { let mut items = DiagnosticItems :: default () ; for cnum in tcx . crates (()) . iter () . copied () . filter (| cnum | tcx . is_user_visible_dep (* cnum)) . chain (std :: iter :: once (LOCAL_CRATE)) { for (& name , & def_id) in & tcx . diagnostic_items (cnum) . name_to_id { collect_item (tcx , & mut items , name , def_id) ; } } items }
};
}
