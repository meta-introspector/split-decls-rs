// Generated macro for impl_89 (impl)
macro_rules! Depcrate_serializeimpl_89 {
() => {
// Module: crate::serialize
// Provides: {"impl_89"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for Hash128 { # [inline] fn encode (& self , s : & mut S) { s . emit_raw_bytes (& self . as_u128 () . to_le_bytes ()) ; } }
};
}
