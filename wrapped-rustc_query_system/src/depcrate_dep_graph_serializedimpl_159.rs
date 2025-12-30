// Generated macro for impl_159 (impl)
macro_rules! Depcrate_dep_graph_serializedimpl_159 {
() => {
// Module: crate::dep_graph::serialized
// Provides: {"impl_159"}
// Dependencies: {}
impl EdgeHeader { # [inline] fn start (self) -> usize { self . repr >> DEP_NODE_WIDTH_BITS } # [inline] fn bytes_per_index (self) -> usize { (self . repr & mask (DEP_NODE_WIDTH_BITS)) + 1 } # [inline] fn mask (self) -> u32 { mask (self . bytes_per_index () * 8) as u32 } }
};
}
