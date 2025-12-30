// Generated macro for UnsupportedSignatureAlgorithmContext (struct)
macro_rules! Depcrate_errorUnsupportedSignatureAlgorithmContext {
() => {
// Module: crate::error
// Provides: {"UnsupportedSignatureAlgorithmContext"}
// Dependencies: {}
# [doc = " Additional context for the `UnsupportedSignatureAlgorithm` error variant."] # [doc = ""] # [doc = " The contents of this type depend on whether the `alloc` feature is enabled."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct UnsupportedSignatureAlgorithmContext { # [doc = " The signature algorithm OID that was unsupported."] # [cfg (feature = "alloc")] pub signature_algorithm_id : Vec < u8 > , # [doc = " Supported algorithms that were available for signature verification."] # [cfg (feature = "alloc")] pub supported_algorithms : Vec < AlgorithmIdentifier > , }
};
}
