// Generated macro for impl_51 (impl)
macro_rules! Depcrate_arcimpl_51 {
() => {
// Module: crate::arc
// Provides: {"impl_51"}
// Dependencies: {}
impl < T : ? Sized + Ord > Ord for Arc < T > { fn cmp (& self , other : & Arc < T >) -> Ordering { (* * self) . cmp (& * * other) } }
};
}
