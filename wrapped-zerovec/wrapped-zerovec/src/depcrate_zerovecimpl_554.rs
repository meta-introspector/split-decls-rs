// Generated macro for impl_554 (impl)
macro_rules! Depcrate_zerovecimpl_554 {
() => {
// Module: crate::zerovec
// Provides: {"impl_554"}
// Dependencies: {}
impl < 'a , T : AsULE + Ord > Ord for ZeroVec < 'a , T > { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
