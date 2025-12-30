// Generated macro for impl_55 (impl)
macro_rules! Depcrate_serializeimpl_55 {
() => {
// Module: crate::serialize
// Provides: {"impl_55"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for path :: Path { fn encode (& self , e : & mut S) { self . to_str () . unwrap () . encode (e) ; } }
};
}
