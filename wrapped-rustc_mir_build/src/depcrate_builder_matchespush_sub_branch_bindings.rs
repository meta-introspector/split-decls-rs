// Generated macro for push_sub_branch_bindings (function)
macro_rules! Depcrate_builder_matchespush_sub_branch_bindings {
() => {
// Module: crate::builder::matches
// Provides: {"push_sub_branch_bindings"}
// Dependencies: {}
# [doc = " Helper for [`sub_branch_bindings`]. Collects bindings from `candidate_bindings` into"] # [doc = " `flattened`. Bindings in or-patterns are collected recursively from `remainder`."] fn push_sub_branch_bindings < 'c , 'tcx : 'c > (flattened : & mut Vec < Binding < 'tcx > > , candidate_bindings : & 'c [SubpatternBindings < 'tcx >] , remainder : & mut impl Iterator < Item = & 'c [SubpatternBindings < 'tcx >] > ,) { for subpat_bindings in candidate_bindings { match subpat_bindings { SubpatternBindings :: One (binding) => flattened . push (* binding) , SubpatternBindings :: FromOrPattern => { if let Some (subcandidate_bindings) = remainder . next () { push_sub_branch_bindings (flattened , subcandidate_bindings , remainder) ; } else { ty :: tls :: with (| tcx | { tcx . dcx () . delayed_bug ("mismatched or-pattern bindings but no error emitted") }) ; } ; } } } }
};
}
