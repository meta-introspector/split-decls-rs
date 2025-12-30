// Generated macro for impl_556 (impl)
macro_rules! Depcrate_zerovecimpl_556 {
() => {
// Module: crate::zerovec
// Provides: {"impl_556"}
// Dependencies: {}
impl < 'a , T : AsULE > From < & 'a [T :: ULE] > for ZeroVec < 'a , T > { fn from (other : & 'a [T :: ULE]) -> Self { ZeroVec :: new_borrowed (other) } }
};
}
