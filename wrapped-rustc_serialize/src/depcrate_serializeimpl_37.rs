// Generated macro for impl_37 (impl)
macro_rules! Depcrate_serializeimpl_37 {
() => {
// Module: crate::serialize
// Provides: {"impl_37"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for Rc < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
};
}
