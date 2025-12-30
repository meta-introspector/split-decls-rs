// Generated macro for impl_357 (impl)
macro_rules! Depcrateimpl_357 {
() => {
// Module: crate
// Provides: {"impl_357"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for Span { fn encode (& self , s : & mut E) { s . encode_span (* self) ; } }
};
}
