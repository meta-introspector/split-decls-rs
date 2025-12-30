// Generated macro for check_private_in_public (function)
macro_rules! Depcratecheck_private_in_public {
() => {
// Module: crate
// Provides: {"check_private_in_public"}
// Dependencies: {}
fn check_private_in_public (tcx : TyCtxt < '_ > , module_def_id : LocalModDefId) { let effective_visibilities = tcx . effective_visibilities (()) ; let checker = PrivateItemsInPublicInterfacesChecker { tcx , effective_visibilities } ; let crate_items = tcx . hir_module_items (module_def_id) ; let _ = crate_items . par_items (| id | Ok (checker . check_item (id))) ; let _ = crate_items . par_foreign_items (| id | Ok (checker . check_foreign_item (id))) ; }
};
}
