// Generated macro for impl_68 (impl)
macro_rules! Depcrate_serializeimpl_68 {
() => {
// Module: crate::serialize
// Provides: {"impl_68"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for ThinVec < T > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
};
}
