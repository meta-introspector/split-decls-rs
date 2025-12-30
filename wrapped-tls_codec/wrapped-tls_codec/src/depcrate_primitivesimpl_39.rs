// Generated macro for impl_39 (impl)
macro_rules! Depcrate_primitivesimpl_39 {
() => {
// Module: crate::primitives
// Provides: {"impl_39"}
// Dependencies: {}
impl < T , U > Size for (T , U) where T : Size , U : Size , { # [inline (always)] fn tls_serialized_len (& self) -> usize { self . 0 . tls_serialized_len () + self . 1 . tls_serialized_len () } }
};
}
