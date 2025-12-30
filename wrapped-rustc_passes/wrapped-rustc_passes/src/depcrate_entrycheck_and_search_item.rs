// Generated macro for check_and_search_item (function)
macro_rules! Depcrate_entrycheck_and_search_item {
() => {
// Module: crate::entry
// Provides: {"check_and_search_item"}
// Dependencies: {}
fn check_and_search_item (id : ItemId , ctxt : & mut EntryContext < '_ >) { if ! matches ! (ctxt . tcx . def_kind (id . owner_id) , DefKind :: Fn) { for attr in [sym :: rustc_main] { if let Some (span) = attr_span_by_symbol (ctxt , id , attr) { ctxt . tcx . dcx () . emit_err (AttrOnlyInFunctions { span , attr }) ; } } return ; } let at_root = ctxt . tcx . opt_local_parent (id . owner_id . def_id) == Some (CRATE_DEF_ID) ; let attrs = ctxt . tcx . hir_attrs (id . hir_id ()) ; let entry_point_type = rustc_ast :: entry :: entry_point_type (attrs , at_root , ctxt . tcx . opt_item_name (id . owner_id . to_def_id ()) ,) ; match entry_point_type { EntryPointType :: None => { } EntryPointType :: MainNamed => { } EntryPointType :: OtherMain => { ctxt . non_main_fns . push (ctxt . tcx . def_span (id . owner_id)) ; } EntryPointType :: RustcMainAttr => { if ctxt . rustc_main_fn . is_none () { ctxt . rustc_main_fn = Some ((id . owner_id . def_id , ctxt . tcx . def_span (id . owner_id))) ; } else { ctxt . tcx . dcx () . emit_err (MultipleRustcMain { span : ctxt . tcx . def_span (id . owner_id . to_def_id ()) , first : ctxt . rustc_main_fn . unwrap () . 1 , additional : ctxt . tcx . def_span (id . owner_id . to_def_id ()) , }) ; } } } }
};
}
