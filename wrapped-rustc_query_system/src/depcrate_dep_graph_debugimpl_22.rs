// Generated macro for impl_22 (impl)
macro_rules! Depcrate_dep_graph_debugimpl_22 {
() => {
// Module: crate::dep_graph::debug
// Provides: {"impl_22"}
// Dependencies: {}
impl EdgeFilter { pub fn new (test : & str) -> Result < EdgeFilter , Box < dyn Error > > { if let [source , target] = * test . split ("->") . collect :: < Vec < _ > > () { Ok (EdgeFilter { source : DepNodeFilter :: new (source) , target : DepNodeFilter :: new (target) , index_to_node : Lock :: new (FxHashMap :: default ()) , }) } else { Err (format ! ("expected a filter like `a&b -> c&d`, not `{test}`") . into ()) } } # [cfg (debug_assertions)] pub fn test (& self , source : & DepNode , target : & DepNode) -> bool { self . source . test (source) && self . target . test (target) } }
};
}
