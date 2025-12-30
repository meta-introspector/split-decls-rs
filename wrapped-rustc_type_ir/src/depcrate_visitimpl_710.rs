// Generated macro for impl_710 (impl)
macro_rules! Depcrate_visitimpl_710 {
() => {
// Module: crate::visit
// Provides: {"impl_710"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Vec < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
};
}
