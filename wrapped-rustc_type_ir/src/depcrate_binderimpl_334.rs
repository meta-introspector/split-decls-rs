// Generated macro for impl_334 (impl)
macro_rules! Depcrate_binderimpl_334 {
() => {
// Module: crate::binder
// Provides: {"impl_334"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeSuperVisitable < I > for Binder < I , T > { fn super_visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { self . as_ref () . skip_binder () . visit_with (visitor) } }
};
}
