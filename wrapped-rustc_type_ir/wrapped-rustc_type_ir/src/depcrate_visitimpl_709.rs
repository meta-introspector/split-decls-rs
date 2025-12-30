// Generated macro for impl_709 (impl)
macro_rules! Depcrate_visitimpl_709 {
() => {
// Module: crate::visit
// Provides: {"impl_709"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Box < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { (* * self) . visit_with (visitor) } }
};
}
