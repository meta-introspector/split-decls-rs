// Generated macro for impl_380 (impl)
macro_rules! Depcrateimpl_380 {
() => {
// Module: crate
// Provides: {"impl_380"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for DefId { fn encode (& self , s : & mut E) { s . encode_def_id (* self) } }
};
}
