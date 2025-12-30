// Generated macro for impl_152 (impl)
macro_rules! Depcrate_opaqueimpl_152 {
() => {
// Module: crate::opaque
// Provides: {"impl_152"}
// Dependencies: {}
impl Encodable < FileEncoder > for [u8] { fn encode (& self , e : & mut FileEncoder) { Encoder :: emit_usize (e , self . len ()) ; e . emit_raw_bytes (self) ; } }
};
}
