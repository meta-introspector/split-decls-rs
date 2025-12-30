// Generated macro for PatternColumn (struct)
macro_rules! Depcrate_pat_columnPatternColumn {
() => {
// Module: crate::pat_column
// Provides: {"PatternColumn"}
// Dependencies: {}
# [doc = " A column of patterns in a match, where a column is the intuitive notion of \"subpatterns that"] # [doc = " inspect the same subvalue/place\"."] # [doc = " This is used to traverse patterns column-by-column for lints. Despite similarities with the"] # [doc = " algorithm in [`crate::usefulness`], this does a different traversal. Notably this is linear in"] # [doc = " the depth of patterns, whereas `compute_exhaustiveness_and_usefulness` is worst-case exponential"] # [doc = " (exhaustiveness is NP-complete). The core difference is that we treat sub-columns separately."] # [doc = ""] # [doc = " This is not used in the usefulness algorithm; only in lints."] # [derive (Debug)] pub struct PatternColumn < 'p , Cx : PatCx > { # [doc = " This must not contain an or-pattern. `expand_and_push` takes care to expand them."] patterns : Vec < & 'p DeconstructedPat < Cx > > , }
};
}
