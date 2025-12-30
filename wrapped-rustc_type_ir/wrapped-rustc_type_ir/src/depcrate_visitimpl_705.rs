// Generated macro for impl_705 (impl)
macro_rules! Depcrate_visitimpl_705 {
() => {
// Module: crate::visit
// Provides: {"impl_705"}
// Dependencies: {}
impl < I : Interner , A : TypeVisitable < I > , B : TypeVisitable < I > , C : TypeVisitable < I > > TypeVisitable < I > for (A , B , C) { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { try_visit ! (self . 0 . visit_with (visitor)) ; try_visit ! (self . 1 . visit_with (visitor)) ; self . 2 . visit_with (visitor) } }
};
}
