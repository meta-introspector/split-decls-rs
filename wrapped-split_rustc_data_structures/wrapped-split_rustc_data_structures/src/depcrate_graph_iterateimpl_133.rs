// Generated macro for impl_133 (impl)
macro_rules! Depcrate_graph_iterateimpl_133 {
() => {
// Module: crate::graph::iterate
// Provides: {"impl_133"}
// Dependencies: {}
impl < G > std :: fmt :: Debug for DepthFirstSearch < G > where G : DirectedGraph + Successors , { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut f = fmt . debug_set () ; for n in self . visited . iter () { f . entry (& n) ; } f . finish () } }
};
}
