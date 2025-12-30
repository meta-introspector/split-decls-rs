// Generated macro for impl_20 (impl)
macro_rules! Depcrate_spannedimpl_20 {
() => {
// Module: crate::spanned
// Provides: {"impl_20"}
// Dependencies: {}
impl < T : Hash > Hash for Spanned < T > { fn hash < H : Hasher > (& self , state : & mut H) { self . value . hash (state) ; } }
};
}
