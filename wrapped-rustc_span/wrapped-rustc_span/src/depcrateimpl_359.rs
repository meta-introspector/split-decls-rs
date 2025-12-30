// Generated macro for impl_359 (impl)
macro_rules! Depcrateimpl_359 {
() => {
// Module: crate
// Provides: {"impl_359"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for ByteSymbol { fn encode (& self , s : & mut E) { s . encode_byte_symbol (* self) ; } }
};
}
