// Generated macro for DepNodeFilter (struct)
macro_rules! Depcrate_dep_graph_debugDepNodeFilter {
() => {
// Module: crate::dep_graph::debug
// Provides: {"DepNodeFilter"}
// Dependencies: {}
# [doc = " A dep-node filter goes from a user-defined string to a query over"] # [doc = " nodes. Right now the format is like this:"] # [doc = " ```ignore (illustrative)"] # [doc = " x & y & z"] # [doc = " ```"] # [doc = " where the format-string of the dep-node must contain `x`, `y`, and"] # [doc = " `z`."] # [derive (Debug)] pub struct DepNodeFilter { text : String , }
};
}
