// Generated macro for impl_84 (impl)
macro_rules! Depcrate_serializeimpl_84 {
() => {
// Module: crate::serialize
// Provides: {"impl_84"}
// Dependencies: {}
impl < E : Encoder , T : Encodable < E > > Encodable < E > for Rc < [T] > { fn encode (& self , s : & mut E) { let slice : & [T] = self ; slice . encode (s) ; } }
};
}
