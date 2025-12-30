// Generated macro for impl_333 (impl)
macro_rules! Depcrate_builder_matchesimpl_333 {
() => {
// Module: crate::builder::matches
// Provides: {"impl_333"}
// Dependencies: {}
impl < 'tcx > MatchTreeBranch < 'tcx > { fn from_candidate (candidate : Candidate < 'tcx >) -> Self { let mut sub_branches = Vec :: new () ; traverse_candidate (candidate , & mut Vec :: new () , & mut | candidate : Candidate < '_ > , parent_data : & mut Vec < PatternExtraData < '_ > > | { sub_branches . push (MatchTreeSubBranch :: from_sub_candidate (candidate , parent_data)) ; } , | inner_candidate , parent_data | { parent_data . push (inner_candidate . extra_data) ; inner_candidate . subcandidates . into_iter () } , | parent_data | { parent_data . pop () ; } ,) ; MatchTreeBranch { sub_branches } } }
};
}
