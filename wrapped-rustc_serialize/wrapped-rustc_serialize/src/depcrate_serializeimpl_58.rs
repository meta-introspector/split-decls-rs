// Generated macro for impl_58 (impl)
macro_rules! Depcrate_serializeimpl_58 {
() => {
// Module: crate::serialize
// Provides: {"impl_58"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > + Copy > Encodable < S > for Cell < T > { fn encode (& self , s : & mut S) { self . get () . encode (s) ; } }
};
}
