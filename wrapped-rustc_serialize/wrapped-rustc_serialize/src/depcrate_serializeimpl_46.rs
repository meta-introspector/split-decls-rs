// Generated macro for impl_46 (impl)
macro_rules! Depcrate_serializeimpl_46 {
() => {
// Module: crate::serialize
// Provides: {"impl_46"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for Cow < '_ , str > { fn encode (& self , s : & mut S) { let val : & str = self ; val . encode (s) } }
};
}
