// Generated macro for impl_181 (impl)
macro_rules! Depcrate_def_idimpl_181 {
() => {
// Module: crate::def_id
// Provides: {"impl_181"}
// Dependencies: {}
impl < D : SpanDecoder > Decodable < D > for LocalDefId { fn decode (d : & mut D) -> LocalDefId { DefId :: decode (d) . expect_local () } }
};
}
