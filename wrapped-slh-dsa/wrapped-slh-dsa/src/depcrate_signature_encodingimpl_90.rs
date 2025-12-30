// Generated macro for impl_90 (impl)
macro_rules! Depcrate_signature_encodingimpl_90 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_90"}
// Dependencies: {}
impl < P : ParameterSet > From < & Array < u8 , P :: SigLen > > for Signature < P > { fn from (bytes : & Array < u8 , P :: SigLen >) -> Signature < P > { Signature :: try_from (bytes . as_slice ()) . unwrap () } }
};
}
