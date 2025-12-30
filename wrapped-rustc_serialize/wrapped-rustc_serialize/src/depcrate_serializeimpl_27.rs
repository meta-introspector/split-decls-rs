// Generated macro for impl_27 (impl)
macro_rules! Depcrate_serializeimpl_27 {
() => {
// Module: crate::serialize
// Provides: {"impl_27"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for NonZero < u32 > { fn encode (& self , s : & mut S) { s . emit_u32 (self . get ()) ; } }
};
}
