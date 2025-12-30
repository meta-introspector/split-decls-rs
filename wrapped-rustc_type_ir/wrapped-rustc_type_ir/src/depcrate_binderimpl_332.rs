// Generated macro for impl_332 (impl)
macro_rules! Depcrate_binderimpl_332 {
() => {
// Module: crate::binder
// Provides: {"impl_332"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Binder < I , T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { visitor . visit_binder (self) } }
};
}
