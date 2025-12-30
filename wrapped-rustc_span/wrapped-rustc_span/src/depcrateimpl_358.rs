// Generated macro for impl_358 (impl)
macro_rules! Depcrateimpl_358 {
() => {
// Module: crate
// Provides: {"impl_358"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for Symbol { fn encode (& self , s : & mut E) { s . encode_symbol (* self) ; } }
};
}
