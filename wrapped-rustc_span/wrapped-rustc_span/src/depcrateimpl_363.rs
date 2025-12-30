// Generated macro for impl_363 (impl)
macro_rules! Depcrateimpl_363 {
() => {
// Module: crate
// Provides: {"impl_363"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for DefIndex { fn encode (& self , s : & mut E) { s . encode_def_index (* self) } }
};
}
