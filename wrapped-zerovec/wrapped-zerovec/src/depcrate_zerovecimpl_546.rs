// Generated macro for impl_546 (impl)
macro_rules! Depcrate_zerovecimpl_546 {
() => {
// Module: crate::zerovec
// Provides: {"impl_546"}
// Dependencies: {}
impl < 'a , T : AsULE > AsRef < ZeroSlice < T > > for ZeroVec < 'a , T > { fn as_ref (& self) -> & ZeroSlice < T > { self . as_slice () } }
};
}
