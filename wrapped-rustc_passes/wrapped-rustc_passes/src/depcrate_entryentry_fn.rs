// Generated macro for entry_fn (function)
macro_rules! Depcrate_entryentry_fn {
() => {
// Module: crate::entry
// Provides: {"entry_fn"}
// Dependencies: {}
fn entry_fn (tcx : TyCtxt < '_ > , () : ()) -> Option < (DefId , EntryFnType) > { let any_exe = tcx . crate_types () . contains (& CrateType :: Executable) ; if ! any_exe { return None ; } if attr :: contains_name (tcx . hir_attrs (CRATE_HIR_ID) , sym :: no_main) { return None ; } let mut ctxt = EntryContext { tcx , rustc_main_fn : None , non_main_fns : Vec :: new () } ; for id in tcx . hir_free_items () { check_and_search_item (id , & mut ctxt) ; } configure_main (tcx , & ctxt) }
};
}
