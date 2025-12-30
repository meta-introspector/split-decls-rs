// Generated macro for check_mod_attrs (function)
macro_rules! Depcrate_check_attrcheck_mod_attrs {
() => {
// Module: crate::check_attr
// Provides: {"check_mod_attrs"}
// Dependencies: {}
fn check_mod_attrs (tcx : TyCtxt < '_ > , module_def_id : LocalModDefId) { let check_attr_visitor = & mut CheckAttrVisitor { tcx , abort : Cell :: new (false) } ; tcx . hir_visit_item_likes_in_module (module_def_id , check_attr_visitor) ; if module_def_id . to_local_def_id () . is_top_level_module () { check_attr_visitor . check_attributes (CRATE_HIR_ID , DUMMY_SP , Target :: Mod , None) ; check_invalid_crate_level_attr (tcx , tcx . hir_krate_attrs ()) ; } if check_attr_visitor . abort . get () { tcx . dcx () . abort_if_errors () } }
};
}
