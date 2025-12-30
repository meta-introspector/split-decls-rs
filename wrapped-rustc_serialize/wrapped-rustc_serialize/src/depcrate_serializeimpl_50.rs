// Generated macro for impl_50 (impl)
macro_rules! Depcrate_serializeimpl_50 {
() => {
// Module: crate::serialize
// Provides: {"impl_50"}
// Dependencies: {}
impl < S : Encoder , T1 : Encodable < S > , T2 : Encodable < S > > Encodable < S > for Result < T1 , T2 > { fn encode (& self , s : & mut S) { match * self { Ok (ref v) => { s . emit_u8 (0) ; v . encode (s) ; } Err (ref v) => { s . emit_u8 (1) ; v . encode (s) ; } } } }
};
}
