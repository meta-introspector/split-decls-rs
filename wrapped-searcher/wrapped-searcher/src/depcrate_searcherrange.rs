// Generated macro for Range (type)
macro_rules! Depcrate_searcherRange {
() => {
// Module: crate::searcher
// Provides: {"Range"}
// Dependencies: {}
# [doc = " We use this type alias since we want the ergonomics of a matcher's `Match`"] # [doc = " type, but in practice, we use it for arbitrary ranges, so give it a more"] # [doc = " accurate name. This is only used in the searcher's internals."] type Range = Match ;
};
}
