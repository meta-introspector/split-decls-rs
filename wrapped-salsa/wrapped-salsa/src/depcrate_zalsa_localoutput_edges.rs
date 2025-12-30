// Generated macro for output_edges (function)
macro_rules! Depcrate_zalsa_localoutput_edges {
() => {
// Module: crate::zalsa_local
// Provides: {"output_edges"}
// Dependencies: {}
# [doc = " Returns the (tracked) outputs that were executed in computing this memoized value."] # [doc = ""] # [doc = " These will always be in execution order."] pub (crate) fn output_edges (input_outputs : & [QueryEdge] ,) -> impl DoubleEndedIterator < Item = DatabaseKeyIndex > + use < '_ > { input_outputs . iter () . filter_map (| & edge | match edge . kind () { QueryEdgeKind :: Output (dependency_index) => Some (dependency_index) , QueryEdgeKind :: Input (_) => None , }) }
};
}
