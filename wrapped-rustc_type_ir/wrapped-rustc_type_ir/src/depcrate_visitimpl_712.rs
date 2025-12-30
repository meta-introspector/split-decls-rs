// Generated macro for impl_712 (impl)
macro_rules! Depcrate_visitimpl_712 {
() => {
// Module: crate::visit
// Provides: {"impl_712"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > , const N : usize > TypeVisitable < I > for SmallVec < [T ; N] > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
};
}
