// Generated macro for impl_64 (impl)
macro_rules! Depcrate_serializeimpl_64 {
() => {
// Module: crate::serialize
// Provides: {"impl_64"}
// Dependencies: {}
impl < S : Encoder , T : ? Sized + Encodable < S > > Encodable < S > for Box < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }
};
}
