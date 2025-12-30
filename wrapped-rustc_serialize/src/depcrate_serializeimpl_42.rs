// Generated macro for impl_42 (impl)
macro_rules! Depcrate_serializeimpl_42 {
() => {
// Module: crate::serialize
// Provides: {"impl_42"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > , const N : usize > Encodable < S > for [T ; N] { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
};
}
