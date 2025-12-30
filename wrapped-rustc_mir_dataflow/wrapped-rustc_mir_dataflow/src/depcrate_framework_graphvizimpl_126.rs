// Generated macro for impl_126 (impl)
macro_rules! Depcrate_framework_graphvizimpl_126 {
() => {
// Module: crate::framework::graphviz
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'tcx , A > dot :: GraphWalk < '_ > for Formatter < '_ , 'tcx , A > where A : Analysis < 'tcx > , { type Node = BasicBlock ; type Edge = CfgEdge ; fn nodes (& self) -> dot :: Nodes < '_ , Self :: Node > { self . body . basic_blocks . indices () . filter (| & idx | self . reachable . contains (idx)) . collect :: < Vec < _ > > () . into () } fn edges (& self) -> dot :: Edges < '_ , Self :: Edge > { self . body . basic_blocks . indices () . flat_map (| bb | dataflow_successors (self . body , bb)) . collect :: < Vec < _ > > () . into () } fn source (& self , edge : & Self :: Edge) -> Self :: Node { edge . source } fn target (& self , edge : & Self :: Edge) -> Self :: Node { self . body [edge . source] . terminator () . successors () . nth (edge . index) . unwrap () } }
};
}
