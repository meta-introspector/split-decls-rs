// Generated macro for try_mark_green (function)
macro_rules! Depcrate_plumbingtry_mark_green {
() => {
// Module: crate::plumbing
// Provides: {"try_mark_green"}
// Dependencies: {}
pub (super) fn try_mark_green < 'tcx > (tcx : TyCtxt < 'tcx > , dep_node : & dep_graph :: DepNode) -> bool { tcx . dep_graph . try_mark_green (QueryCtxt :: new (tcx) , dep_node) . is_some () }
};
}
