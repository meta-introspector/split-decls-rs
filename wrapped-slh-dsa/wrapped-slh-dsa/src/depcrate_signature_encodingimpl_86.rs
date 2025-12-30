// Generated macro for impl_86 (impl)
macro_rules! Depcrate_signature_encodingimpl_86 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_86"}
// Dependencies: {}
impl < P : ParameterSet > SignatureEncoding for Signature < P > { type Repr = Array < u8 , P :: SigLen > ; fn encoded_len (& self) -> usize { P :: SigLen :: USIZE } }
};
}
