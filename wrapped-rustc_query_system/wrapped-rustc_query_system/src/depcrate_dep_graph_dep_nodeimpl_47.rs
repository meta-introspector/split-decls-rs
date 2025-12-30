// Generated macro for impl_47 (impl)
macro_rules! Depcrate_dep_graph_dep_nodeimpl_47 {
() => {
// Module: crate::dep_graph::dep_node
// Provides: {"impl_47"}
// Dependencies: {}
impl WorkProductId { pub fn from_cgu_name (cgu_name : & str) -> WorkProductId { let mut hasher = StableHasher :: new () ; cgu_name . hash (& mut hasher) ; WorkProductId { hash : hasher . finish () } } }
};
}
