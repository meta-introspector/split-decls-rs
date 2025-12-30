// Generated macro for impl_56 (impl)
macro_rules! Depcrate_serializeimpl_56 {
() => {
// Module: crate::serialize
// Provides: {"impl_56"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for path :: PathBuf { fn encode (& self , e : & mut S) { path :: Path :: encode (self , e) ; } }
};
}
