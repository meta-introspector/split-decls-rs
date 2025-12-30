// Generated macro for impl_704 (impl)
macro_rules! Depcrate_visitimpl_704 {
() => {
// Module: crate::visit
// Provides: {"impl_704"}
// Dependencies: {}
impl < I : Interner , T : TypeVisitable < I > , U : TypeVisitable < I > > TypeVisitable < I > for (T , U) { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { try_visit ! (self . 0 . visit_with (visitor)) ; self . 1 . visit_with (visitor) } }
};
}
