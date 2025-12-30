// Generated macro for other_723 (other)
macro_rules! Depcrate_zalsa_localother_723 {
() => {
// Module: crate::zalsa_local
// Provides: {"other_723"}
// Dependencies: {}
# [doc = " The data portion of `PackedQueryOrigin`."] union QueryOriginData { # [doc = " Query edges for `QueryOriginKind::Derived` or `QueryOriginKind::DerivedUntracked`."] # [doc = ""] # [doc = " The query edges are between a memoized value and other queries in the dependency graph,"] # [doc = " including both dependency edges (e.g., when creating the memoized value for Q0"] # [doc = " executed another function Q1) and output edges (e.g., when Q0 specified the value"] # [doc = " for another query Q2)."] # [doc = ""] # [doc = " Note that we always track input dependencies even when there are untracked reads."] # [doc = " Untracked reads mean that Salsa can't verify values, so the list of inputs is unused."] # [doc = " However, Salsa still uses these edges to find the transitive inputs to an accumulator."] # [doc = ""] # [doc = " You can access the input/output list via the methods [`inputs`] and [`outputs`] respectively."] # [doc = ""] # [doc = " Important:"] # [doc = ""] # [doc = " * The inputs must be in **execution order** for the red-green algorithm to work."] input_outputs : NonNull < QueryEdge > , # [doc = " The identity of the assigning query for `QueryOriginKind::Assigned`."] index : Id , # [doc = " `QueryOriginKind::FixpointInitial` holds no data."] empty : () , }
};
}
