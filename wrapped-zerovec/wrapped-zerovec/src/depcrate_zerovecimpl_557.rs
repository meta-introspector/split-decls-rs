// Generated macro for impl_557 (impl)
macro_rules! Depcrate_zerovecimpl_557 {
() => {
// Module: crate::zerovec
// Provides: {"impl_557"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T : AsULE > From < Vec < T :: ULE > > for ZeroVec < 'a , T > { fn from (other : Vec < T :: ULE >) -> Self { ZeroVec :: new_owned (other) } }
};
}
