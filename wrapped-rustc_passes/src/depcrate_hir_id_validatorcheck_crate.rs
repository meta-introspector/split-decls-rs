// Generated macro for check_crate (function)
macro_rules! Depcrate_hir_id_validatorcheck_crate {
() => {
// Module: crate::hir_id_validator
// Provides: {"check_crate"}
// Dependencies: {}
pub fn check_crate (tcx : TyCtxt < '_ >) { let errors = Lock :: new (Vec :: new ()) ; tcx . par_hir_for_each_module (| module_id | { let mut v = HirIdValidator { tcx , owner : None , hir_ids_seen : Default :: default () , errors : & errors } ; tcx . hir_visit_item_likes_in_module (module_id , & mut v) ; }) ; let errors = errors . into_inner () ; if ! errors . is_empty () { let message = errors . iter () . fold (String :: new () , | s1 , s2 | s1 + "\n" + s2) ; tcx . dcx () . delayed_bug (message) ; } }
};
}
