// Generated macro for impl_376 (impl)
macro_rules! Depcrateimpl_376 {
() => {
// Module: crate
// Provides: {"impl_376"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for ExpnId { fn encode (& self , s : & mut E) { s . encode_expn_id (* self) } }
};
}
