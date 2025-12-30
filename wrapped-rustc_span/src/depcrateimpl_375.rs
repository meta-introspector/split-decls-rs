// Generated macro for impl_375 (impl)
macro_rules! Depcrateimpl_375 {
() => {
// Module: crate
// Provides: {"impl_375"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for ByteSymbol { fn encode (& self , s : & mut E) { s . encode_byte_symbol (* self) ; } }
};
}
