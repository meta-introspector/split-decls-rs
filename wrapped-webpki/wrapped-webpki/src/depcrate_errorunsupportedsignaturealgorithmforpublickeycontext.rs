// Generated macro for UnsupportedSignatureAlgorithmForPublicKeyContext (struct)
macro_rules! Depcrate_errorUnsupportedSignatureAlgorithmForPublicKeyContext {
() => {
// Module: crate::error
// Provides: {"UnsupportedSignatureAlgorithmForPublicKeyContext"}
// Dependencies: {}
# [doc = " Additional context for the `UnsupportedSignatureAlgorithmForPublicKey` error variant."] # [doc = ""] # [doc = " The contents of this type depend on whether the `alloc` feature is enabled."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct UnsupportedSignatureAlgorithmForPublicKeyContext { # [doc = " The signature algorithm OID."] # [cfg (feature = "alloc")] pub signature_algorithm_id : Vec < u8 > , # [doc = " The public key algorithm OID."] # [cfg (feature = "alloc")] pub public_key_algorithm_id : Vec < u8 > , }
};
}
