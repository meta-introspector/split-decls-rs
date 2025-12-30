// Generated macro for impl_24 (impl)
macro_rules! Depcrate_serializeimpl_24 {
() => {
// Module: crate::serialize
// Provides: {"impl_24"}
// Dependencies: {}
impl < S : Encoder , T : ? Sized + PointeeSized > Encodable < S > for & T where T : Encodable < S > , { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }
};
}
