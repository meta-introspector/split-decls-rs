// Generated macro for impl_21 (impl)
macro_rules! Depcrate_primitivesimpl_21 {
() => {
// Module: crate::primitives
// Provides: {"impl_21"}
// Dependencies: {}
impl < T : Size > Size for Option < T > { # [inline] fn tls_serialized_len (& self) -> usize { 1 + match self { Some (v) => v . tls_serialized_len () , None => 0 , } } }
};
}
