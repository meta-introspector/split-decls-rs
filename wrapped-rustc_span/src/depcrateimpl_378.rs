// Generated macro for impl_378 (impl)
macro_rules! Depcrateimpl_378 {
() => {
// Module: crate
// Provides: {"impl_378"}
// Dependencies: {}
impl < E : SpanEncoder > Encodable < E > for CrateNum { fn encode (& self , s : & mut E) { s . encode_crate_num (* self) } }
};
}
