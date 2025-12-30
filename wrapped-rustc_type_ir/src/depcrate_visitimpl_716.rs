// Generated macro for impl_716 (impl)
macro_rules! Depcrate_visitimpl_716 {
() => {
// Module: crate::visit
// Provides: {"impl_716"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > , S > TypeVisitable < I > for indexmap :: IndexSet < T , S > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
};
}
