// Generated macro for is_unstable_reexport (function)
macro_rules! Depcrate_stabilityis_unstable_reexport {
() => {
// Module: crate::stability
// Provides: {"is_unstable_reexport"}
// Dependencies: {}
# [doc = " Check whether a path is a `use` item that has been marked as unstable."] # [doc = ""] # [doc = " See issue #94972 for details on why this is a special case"] fn is_unstable_reexport (tcx : TyCtxt < '_ > , id : hir :: HirId) -> bool { let Some (owner) = id . as_owner () else { return false ; } ; let def_id = owner . def_id ; let Some (stab) = tcx . lookup_stability (def_id) else { return false ; } ; if stab . level . is_stable () { return false ; } if ! matches ! (tcx . hir_expect_item (def_id) . kind , ItemKind :: Use (..)) { return false ; } true }
};
}
