// Generated macro for impl_25 (impl)
macro_rules! Depcrate_primitivesimpl_25 {
() => {
// Module: crate::primitives
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : Serialize > Serialize for & Option < T > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { (* self) . tls_serialize (writer) } }
};
}
