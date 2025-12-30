// Generated macro for impl_54 (impl)
macro_rules! Depcrate_primitivesimpl_54 {
() => {
// Module: crate::primitives
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : Serialize > Serialize for Box < T > { # [cfg (feature = "std")] # [inline (always)] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . as_ref () . tls_serialize (writer) } }
};
}
