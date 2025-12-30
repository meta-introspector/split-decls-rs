// Generated macro for FlatPat (struct)
macro_rules! Depcrate_builder_matchesFlatPat {
() => {
// Module: crate::builder::matches
// Provides: {"FlatPat"}
// Dependencies: {}
# [doc = " A pattern in a form suitable for lowering the match tree, with all irrefutable"] # [doc = " patterns simplified away."] # [doc = ""] # [doc = " Here, \"flat\" indicates that irrefutable nodes in the pattern tree have been"] # [doc = " recursively replaced with their refutable subpatterns. They are not"] # [doc = " necessarily flat in an absolute sense."] # [doc = ""] # [doc = " Will typically be incorporated into a [`Candidate`]."] # [derive (Debug , Clone)] struct FlatPat < 'tcx > { # [doc = " To match the pattern, all of these must be satisfied..."] match_pairs : Vec < MatchPairTree < 'tcx > > , extra_data : PatternExtraData < 'tcx > , }
};
}
