// Generated macro for impl_715 (impl)
macro_rules! Depcrate_visitimpl_715 {
() => {
// Module: crate::visit
// Provides: {"impl_715"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > , Ix : Idx > TypeVisitable < I > for IndexVec < Ix , T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
};
}
