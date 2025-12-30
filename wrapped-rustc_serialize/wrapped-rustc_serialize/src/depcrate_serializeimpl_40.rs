// Generated macro for impl_40 (impl)
macro_rules! Depcrate_serializeimpl_40 {
() => {
// Module: crate::serialize
// Provides: {"impl_40"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for Vec < T > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
};
}
