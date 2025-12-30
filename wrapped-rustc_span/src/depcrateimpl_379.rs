// Generated macro for impl_379 (impl)
macro_rules! Depcrateimpl_379 {
() => {
// Module: crate
// Provides: {"impl_379"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for DefIndex { fn encode (& self , s : & mut E) { s . encode_def_index (* self) } }
};
}
