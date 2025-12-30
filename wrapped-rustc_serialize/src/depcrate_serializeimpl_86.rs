// Generated macro for impl_86 (impl)
macro_rules! Depcrate_serializeimpl_86 {
() => {
// Module: crate::serialize
// Provides: {"impl_86"}
// Dependencies: {}
impl < E : Encoder , T : Encodable < E > > Encodable < E > for Arc < [T] > { fn encode (& self , s : & mut E) { let slice : & [T] = self ; slice . encode (s) ; } }
};
}
