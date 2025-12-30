// Generated macro for impl_706 (impl)
macro_rules! Depcrate_visitimpl_706 {
() => {
// Module: crate::visit
// Provides: {"impl_706"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Option < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { match self { Some (v) => v . visit_with (visitor) , None => V :: Result :: output () , } } }
};
}
