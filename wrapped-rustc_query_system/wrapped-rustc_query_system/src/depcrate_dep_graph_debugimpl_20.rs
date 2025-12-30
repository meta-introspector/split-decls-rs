// Generated macro for impl_20 (impl)
macro_rules! Depcrate_dep_graph_debugimpl_20 {
() => {
// Module: crate::dep_graph::debug
// Provides: {"impl_20"}
// Dependencies: {}
impl DepNodeFilter { pub fn new (text : & str) -> Self { DepNodeFilter { text : text . trim () . to_string () } } # [doc = " Returns `true` if all nodes always pass the filter."] pub fn accepts_all (& self) -> bool { self . text . is_empty () } # [doc = " Tests whether `node` meets the filter, returning true if so."] pub fn test (& self , node : & DepNode) -> bool { let debug_str = format ! ("{node:?}") ; self . text . split ('&') . map (| s | s . trim ()) . all (| f | debug_str . contains (f)) } }
};
}
