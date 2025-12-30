// Generated macro for impl_332 (impl)
macro_rules! Depcrate_builder_matchesimpl_332 {
() => {
// Module: crate::builder::matches
// Provides: {"impl_332"}
// Dependencies: {}
impl < 'tcx > MatchTreeSubBranch < 'tcx > { fn from_sub_candidate (candidate : Candidate < 'tcx > , parent_data : & Vec < PatternExtraData < 'tcx > > ,) -> Self { debug_assert ! (candidate . match_pairs . is_empty ()) ; MatchTreeSubBranch { span : candidate . extra_data . span , success_block : candidate . pre_binding_block . unwrap () , otherwise_block : candidate . otherwise_block . unwrap () , bindings : sub_branch_bindings (parent_data , & candidate . extra_data . bindings) , ascriptions : parent_data . iter () . flat_map (| d | & d . ascriptions) . cloned () . chain (candidate . extra_data . ascriptions) . collect () , is_never : candidate . extra_data . is_never , } } }
};
}
