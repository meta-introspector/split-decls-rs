// Generated macro for impl_396 (impl)
macro_rules! Depcrate_obligation_forest_graphvizimpl_396 {
() => {
// Module: crate::obligation_forest::graphviz
// Provides: {"impl_396"}
// Dependencies: {}
impl < 'a , O : ForestObligation + 'a > dot :: GraphWalk < 'a > for & 'a ObligationForest < O > { type Node = usize ; type Edge = (usize , usize) ; fn nodes (& self) -> dot :: Nodes < '_ , Self :: Node > { (0 .. self . nodes . len ()) . collect () } fn edges (& self) -> dot :: Edges < '_ , Self :: Edge > { (0 .. self . nodes . len ()) . flat_map (| i | { let node = & self . nodes [i] ; node . dependents . iter () . map (move | & d | (d , i)) }) . collect () } fn source (& self , (s , _) : & Self :: Edge) -> Self :: Node { * s } fn target (& self , (_ , t) : & Self :: Edge) -> Self :: Node { * t } }
};
}
