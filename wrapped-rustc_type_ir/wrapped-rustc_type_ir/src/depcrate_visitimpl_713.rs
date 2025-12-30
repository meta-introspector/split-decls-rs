// Generated macro for impl_713 (impl)
macro_rules! Depcrate_visitimpl_713 {
() => {
// Module: crate::visit
// Provides: {"impl_713"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for & [T] { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
};
}
