// Generated macro for impl_364 (impl)
macro_rules! Depcrateimpl_364 {
() => {
// Module: crate
// Provides: {"impl_364"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for DefId { fn encode (& self , s : & mut E) { s . encode_def_id (* self) } }
};
}
