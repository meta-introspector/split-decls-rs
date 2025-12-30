// Generated macro for impl_53 (impl)
macro_rules! Depcrate_fingerprintimpl_53 {
() => {
// Module: crate::fingerprint
// Provides: {"impl_53"}
// Dependencies: {}
impl < E : Encoder > Encodable < E > for Fingerprint { # [inline] fn encode (& self , s : & mut E) { s . emit_raw_bytes (& self . to_le_bytes ()) ; } }
};
}
