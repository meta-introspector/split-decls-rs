// Generated macro for impl_62 (impl)
macro_rules! Depcrate_serializeimpl_62 {
() => {
// Module: crate::serialize
// Provides: {"impl_62"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for Arc < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
};
}
