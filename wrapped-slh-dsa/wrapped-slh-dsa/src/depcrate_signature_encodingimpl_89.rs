// Generated macro for impl_89 (impl)
macro_rules! Depcrate_signature_encodingimpl_89 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_89"}
// Dependencies: {}
impl < P : ParameterSet > From < Signature < P > > for Array < u8 , P :: SigLen > { fn from (sig : Signature < P >) -> Array < u8 , P :: SigLen > { sig . to_bytes () } }
};
}
