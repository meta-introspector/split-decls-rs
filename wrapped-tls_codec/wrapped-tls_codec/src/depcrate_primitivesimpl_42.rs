// Generated macro for impl_42 (impl)
macro_rules! Depcrate_primitivesimpl_42 {
() => {
// Module: crate::primitives
// Provides: {"impl_42"}
// Dependencies: {}
impl < T , U , V > Serialize for (T , U , V) where T : Serialize , U : Serialize , V : Serialize , { # [cfg (feature = "std")] # [inline (always)] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { let mut written = self . 0 . tls_serialize (writer) ? ; written += self . 1 . tls_serialize (writer) ? ; self . 2 . tls_serialize (writer) . map (| l | l + written) } }
};
}
