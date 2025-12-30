// Generated macro for impl_87 (impl)
macro_rules! Depcrate_boundimpl_87 {
() => {
// Module: crate::bound
// Provides: {"impl_87"}
// Dependencies: {}
impl Drop for BoundsChild < '_ > { fn drop (& mut self) { if self . owner . can_extend { self . owner . ty . append (& mut self . bounds . ty) ; self . owner . pred . append (& mut self . bounds . pred) ; } } }
};
}
