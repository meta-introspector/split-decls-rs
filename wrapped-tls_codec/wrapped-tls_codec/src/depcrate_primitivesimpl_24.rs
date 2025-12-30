// Generated macro for impl_24 (impl)
macro_rules! Depcrate_primitivesimpl_24 {
() => {
// Module: crate::primitives
// Provides: {"impl_24"}
// Dependencies: {}
impl < T : SerializeBytes > SerializeBytes for Option < T > { # [inline] fn tls_serialize (& self) -> Result < Vec < u8 > , Error > { match self { Some (e) => { let mut out = Vec :: with_capacity (e . tls_serialized_len () + 1) ; out . push (1) ; out . append (& mut e . tls_serialize () ?) ; Ok (out) } None => Ok (vec ! [0]) , } } }
};
}
