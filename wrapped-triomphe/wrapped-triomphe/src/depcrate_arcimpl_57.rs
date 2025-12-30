// Generated macro for impl_57 (impl)
macro_rules! Depcrate_arcimpl_57 {
() => {
// Module: crate::arc
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : ? Sized + Hash > Hash for Arc < T > { fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) } }
};
}
