// Generated macro for impl_148 (impl)
macro_rules! Depcrate_framework_latticeimpl_148 {
() => {
// Module: crate::framework::lattice
// Provides: {"impl_148"}
// Dependencies: {}
impl < T > HasBottom for FlatSet < T > { const BOTTOM : Self = Self :: Bottom ; fn is_bottom (& self) -> bool { matches ! (self , Self :: Bottom) } }
};
}
