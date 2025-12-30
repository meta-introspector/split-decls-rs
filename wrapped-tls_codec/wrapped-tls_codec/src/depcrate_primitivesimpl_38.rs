// Generated macro for impl_38 (impl)
macro_rules! Depcrate_primitivesimpl_38 {
() => {
// Module: crate::primitives
// Provides: {"impl_38"}
// Dependencies: {}
impl < T , U > Serialize for (T , U) where T : Serialize , U : Serialize , { # [cfg (feature = "std")] # [inline (always)] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { let written = self . 0 . tls_serialize (writer) ? ; self . 1 . tls_serialize (writer) . map (| l | l + written) } }
};
}
