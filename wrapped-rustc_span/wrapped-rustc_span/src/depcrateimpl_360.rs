// Generated macro for impl_360 (impl)
macro_rules! Depcrateimpl_360 {
() => {
// Module: crate
// Provides: {"impl_360"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for ExpnId { fn encode (& self , s : & mut E) { s . encode_expn_id (* self) } }
};
}
