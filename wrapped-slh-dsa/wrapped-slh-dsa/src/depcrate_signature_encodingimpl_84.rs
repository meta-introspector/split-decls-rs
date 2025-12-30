// Generated macro for impl_84 (impl)
macro_rules! Depcrate_signature_encodingimpl_84 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P : ParameterSet > From < & Signature < P > > for alloc :: vec :: Vec < u8 > { fn from (sig : & Signature < P >) -> alloc :: vec :: Vec < u8 > { sig . to_vec () } }
};
}
