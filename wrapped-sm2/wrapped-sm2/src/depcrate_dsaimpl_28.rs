// Generated macro for impl_28 (impl)
macro_rules! Depcrate_dsaimpl_28 {
() => {
// Module: crate::dsa
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl SignatureBitStringEncoding for Signature { fn to_bitstring (& self) -> der_core :: Result < BitString > { BitString :: new (0 , self . to_vec ()) } }
};
}
