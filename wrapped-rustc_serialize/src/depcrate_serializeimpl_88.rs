// Generated macro for impl_88 (impl)
macro_rules! Depcrate_serializeimpl_88 {
() => {
// Module: crate::serialize
// Provides: {"impl_88"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for Hash64 { # [inline] fn encode (& self , s : & mut S) { s . emit_raw_bytes (& self . as_u64 () . to_le_bytes ()) ; } }
};
}
