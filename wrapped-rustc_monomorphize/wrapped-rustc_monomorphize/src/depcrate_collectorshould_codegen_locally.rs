// Generated macro for should_codegen_locally (function)
macro_rules! Depcrate_collectorshould_codegen_locally {
() => {
// Module: crate::collector
// Provides: {"should_codegen_locally"}
// Dependencies: {}
# [doc = " Returns `true` if we should codegen an instance in the local crate, or returns `false` if we"] # [doc = " can just link to the upstream crate and therefore don't need a mono item."] fn should_codegen_locally < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) -> bool { let Some (def_id) = instance . def . def_id_if_not_guaranteed_local_codegen () else { return true ; } ; if tcx . is_foreign_item (def_id) { return false ; } if tcx . def_kind (def_id) . has_codegen_attrs () && matches ! (tcx . codegen_fn_attrs (def_id) . inline , InlineAttr :: Force { .. }) { tcx . dcx () . delayed_bug ("attempt to codegen `#[rustc_force_inline]` item") ; } if def_id . is_local () { return true ; } if tcx . is_reachable_non_generic (def_id) || instance . upstream_monomorphization (tcx) . is_some () { return false ; } if let DefKind :: Static { .. } = tcx . def_kind (def_id) { return false ; } if ! tcx . is_mir_available (def_id) { tcx . dcx () . emit_fatal (NoOptimizedMir { span : tcx . def_span (def_id) , crate_name : tcx . crate_name (def_id . krate) , instance : instance . to_string () , }) ; } true }
};
}
