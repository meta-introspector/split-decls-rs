// Generated macro for impl_362 (impl)
macro_rules! Depcrateimpl_362 {
() => {
// Module: crate
// Provides: {"impl_362"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for CrateNum { fn encode (& self , s : & mut E) { s . encode_crate_num (* self) } }
};
}
