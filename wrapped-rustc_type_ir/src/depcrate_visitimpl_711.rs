// Generated macro for impl_711 (impl)
macro_rules! Depcrate_visitimpl_711 {
() => {
// Module: crate::visit
// Provides: {"impl_711"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for ThinVec < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
};
}
