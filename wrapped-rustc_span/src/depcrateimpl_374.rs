// Generated macro for impl_374 (impl)
macro_rules! Depcrateimpl_374 {
() => {
// Module: crate
// Provides: {"impl_374"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for Symbol { fn encode (& self , s : & mut E) { s . encode_symbol (* self) ; } }
};
}
