// Generated macro for impl_43 (impl)
macro_rules! Depcrate_primitivesimpl_43 {
() => {
// Module: crate::primitives
// Provides: {"impl_43"}
// Dependencies: {}
impl < T , U , V > Size for (T , U , V) where T : Size , U : Size , V : Size , { # [inline (always)] fn tls_serialized_len (& self) -> usize { self . 0 . tls_serialized_len () + self . 1 . tls_serialized_len () + self . 2 . tls_serialized_len () } }
};
}
