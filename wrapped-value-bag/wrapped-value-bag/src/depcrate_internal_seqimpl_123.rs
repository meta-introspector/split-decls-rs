// Generated macro for impl_123 (impl)
macro_rules! Depcrate_internal_seqimpl_123 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a > Seq for dyn DowncastSeq + Send + Sync + 'a { fn visit (& self , visitor : & mut dyn Visitor < '_ >) { self . as_super () . visit (visitor) } }
};
}
