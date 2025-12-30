// Generated macro for impl_100 (impl)
macro_rules! Depcrate_setimpl_100 {
() => {
// Module: crate::set
// Provides: {"impl_100"}
// Dependencies: {}
impl < T : SetMember > fmt :: Debug for Set < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
