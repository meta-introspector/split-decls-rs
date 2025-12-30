// Generated macro for impl_161 (impl)
macro_rules! Depcrate_utilsimpl_161 {
() => {
// Module: crate::utils
// Provides: {"impl_161"}
// Dependencies: {}
impl < T : DiffableStr + ? Sized > Index < Range < usize > > for SliceRemapper < '_ , T > { type Output = T ; fn index (& self , range : Range < usize >) -> & Self :: Output { self . slice (range) . expect ("out of bounds") } }
};
}
