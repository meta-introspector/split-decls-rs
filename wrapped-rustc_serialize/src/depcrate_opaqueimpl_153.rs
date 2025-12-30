// Generated macro for impl_153 (impl)
macro_rules! Depcrate_opaqueimpl_153 {
() => {
// Module: crate::opaque
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a > Decodable < MemDecoder < 'a > > for Vec < u8 > { fn decode (d : & mut MemDecoder < 'a >) -> Self { let len = Decoder :: read_usize (d) ; d . read_raw_bytes (len) . to_owned () } }
};
}
