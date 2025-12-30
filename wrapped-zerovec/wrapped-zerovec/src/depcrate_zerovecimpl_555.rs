// Generated macro for impl_555 (impl)
macro_rules! Depcrate_zerovecimpl_555 {
() => {
// Module: crate::zerovec
// Provides: {"impl_555"}
// Dependencies: {}
impl < 'a , T : AsULE > AsRef < [T :: ULE] > for ZeroVec < 'a , T > { fn as_ref (& self) -> & [T :: ULE] { self . as_ule_slice () } }
};
}
