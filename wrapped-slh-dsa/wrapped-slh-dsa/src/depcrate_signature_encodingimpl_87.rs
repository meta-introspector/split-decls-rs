// Generated macro for impl_87 (impl)
macro_rules! Depcrate_signature_encodingimpl_87 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P : ParameterSet > SignatureBitStringEncoding for Signature < P > { fn to_bitstring (& self) -> der :: Result < BitString > { BitString :: new (0 , self . to_vec ()) } }
};
}
