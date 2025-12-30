// Generated macro for impl_708 (impl)
macro_rules! Depcrate_visitimpl_708 {
() => {
// Module: crate::visit
// Provides: {"impl_708"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Arc < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { (* * self) . visit_with (visitor) } }
};
}
