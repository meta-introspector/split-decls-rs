// Generated macro for impl_373 (impl)
macro_rules! Depcrateimpl_373 {
() => {
// Module: crate
// Provides: {"impl_373"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for Span { fn encode (& self , s : & mut E) { s . encode_span (* self) ; } }
};
}
