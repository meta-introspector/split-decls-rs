// Generated macro for impl_541 (impl)
macro_rules! Depcrate_zerovecimpl_541 {
() => {
// Module: crate::zerovec
// Provides: {"impl_541"}
// Dependencies: {}
impl < 'a , T : AsULE > Deref for ZeroVec < 'a , T > { type Target = ZeroSlice < T > ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
};
}
