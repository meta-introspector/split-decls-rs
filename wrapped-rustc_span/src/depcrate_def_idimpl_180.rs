// Generated macro for impl_180 (impl)
macro_rules! Depcrate_def_idimpl_180 {
() => {
// Module: crate::def_id
// Provides: {"impl_180"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for LocalDefId { fn encode (& self , s : & mut E) { self . to_def_id () . encode (s) ; } }
};
}
