// Generated macro for impl_553 (impl)
macro_rules! Depcrate_zerovecimpl_553 {
() => {
// Module: crate::zerovec
// Provides: {"impl_553"}
// Dependencies: {}
impl < 'a , T : AsULE + PartialOrd > PartialOrd for ZeroVec < 'a , T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
